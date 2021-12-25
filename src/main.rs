mod config;
use anyhow;
use config::Config;
use hyper::header::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

async fn catch_all(req: Request<Body>, config: Arc<Config>) -> Result<Response<Body>, Infallible> {
    log::info!("handling request: {}", req.uri());

    let request_path = String::from(req.uri().path());

    for directory in &config.directories {
        if request_path.starts_with(&directory.prefix) {
            let uri_path = Path::new(&request_path);
            let path = String::from(
                Path::new(&directory.path)
                    .join(uri_path.strip_prefix(&directory.prefix).unwrap())
                    .to_str()
                    .unwrap(),
            );

            if !Path::new(&path).is_file() {
                log::debug!("path \"{}\" is not a file ... skipping", path);
                continue;
            }

            log::debug!("checking path: {}", path);

            match File::open(path.clone()).await {
                Ok(file) => {
                    log::info!("sending \"{}\"", path);
                    let stream = FramedRead::new(file, BytesCodec::new());
                    let body = Body::wrap_stream(stream);
                    let mut resp = Response::new(body);

                    match &*get_file_extension(&path) {
                        "js" => {
                            resp.headers_mut().insert(
                                "Content-Type",
                                HeaderValue::from_str("text/javascript").unwrap(),
                            );
                        }
                        "wasm" => {
                            resp.headers_mut().insert(
                                "Content-Type",
                                HeaderValue::from_str("application/wasm").unwrap(),
                            );
                        }
                        _ => {}
                    }

                    return Ok(resp);
                }
                Err(_e) => {
                    log::debug!("did not find file ... continuing ...");
                }
            }
        }
    }

    log::debug!("could not resolve path ... searching /index.html");

    match (&config.directories)
        .into_iter()
        .find(|dir| dir.prefix == "/")
    {
        Some(dir) => {
            log::debug!("found /");
            let path = String::from(Path::new(&dir.path).join("index.html").to_str().unwrap());
            match File::open(path.clone()).await {
                Ok(file) => {
                    log::info!(
                        "could not resolve \"{}\", sending \"{}\"",
                        request_path,
                        path
                    );
                    let stream = FramedRead::new(file, BytesCodec::new());
                    let body = Body::wrap_stream(stream);
                    return Ok(Response::new(body));
                }
                Err(_e) => {
                    log::debug!("could not find /index.html");
                }
            }
        }
        None => log::debug!("could not find /"),
    }

    log::warn!("could not resolve \"{}\", sending 404", request_path);
    Ok(not_found())
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("".into())
        .unwrap()
}

fn get_file_extension(path: &str) -> String {
    match path.rsplit_once(".") {
        Some((_, ext)) => String::from(ext),
        None => String::new(),
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Arc::new(get_config().await.unwrap());
    let addr: SocketAddr = format!("{}:{}", config.ip, config.port).parse().unwrap();

    let make_svc = make_service_fn(move |_| {
        let config = Arc::clone(&config);

        async { Ok::<_, Infallible>(service_fn(move |req| catch_all(req, config.clone()))) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    log::info!("starting server at {}:{}", addr.ip(), addr.port());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn get_config() -> anyhow::Result<Config> {
    let config = tokio::fs::read_to_string(std::env::var("PWA_SERVER_CONFIG")?).await?;

    serde_json::from_str(&config)
        .map_err(|e| anyhow::anyhow!("error reading configuration file: {:?}", e))
}
