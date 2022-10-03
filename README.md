[![CI](https://github.com/hannes-hochreiner/pwa-server/actions/workflows/main.yml/badge.svg)](https://github.com/hannes-hochreiner/pwa-server/actions/workflows/main.yml)

# PWA-Server

A server for progressive web apps.

## Configuration

PWA-Server expects the following environment variables:

|Name | Description|
|----|----|
|PWA_SERVER_CONFIG| location of the server configuration file|
|RUST_LOG| log level (debug, info, error)|

The server configuration file has the following format:

```json
{
  "ip": "127.0.0.1",
  "port": 8000,
  "directories": [
    {
      "prefix": "/config/",
      "path": "./test_data/config_dir"
    },
    {
      "prefix": "/",
      "path": "./test_data/root_dir"
    }
  ]
}
```

If a request is received, it will be resolved against the specified directories (in the order they are listed).

* substitute the ``prefix`` by the ``path``
* if the resulting path points to a file, send that file
* if after processing all ``directories`` no file was sent, search for a directory with the ``prefix`` "/".
* if a directory with the ``prefix`` "/" is found, check whether the file "index.html" exists at the ``path``.
* if "index.html" was found, send it
* send a 404 error in all other cases

## License

This work is licensed under the MIT license.

`SPDX-License-Identifier: MIT`
