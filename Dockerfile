FROM fedora:36 AS builder
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN dnf install gcc openssl-devel -y
RUN mkdir -p /opt/pwa-server
COPY src /opt/pwa-server/src
COPY Cargo.* /opt/pwa-server/
RUN source $HOME/.cargo/env && cd /opt/pwa-server && cargo build --release

FROM fedora:34
MAINTAINER Hannes Hochreiner <hannes@hochreiner.net>
COPY --from=builder /opt/pwa-server/target/release/pwa-server /opt/pwa-server
CMD ["/opt/pwa-server"]