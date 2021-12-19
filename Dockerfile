FROM rust:slim AS builder
RUN rustup default stable
RUN mkdir -p /opt/pwa-server
COPY src /opt/pwa-server/src
COPY Cargo.* /opt/pwa-server/
RUN cd /opt/pwa-server && cargo build --release

FROM debian:stable-slim AS rss-json-service
MAINTAINER Hannes Hochreiner <hannes@hochreiner.net>
COPY --from=builder /opt/pwa-server/target/release/pwa-server /opt/pwa-server
CMD ["/opt/pwa-server"]