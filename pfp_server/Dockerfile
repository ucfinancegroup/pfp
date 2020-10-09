# https://hub.docker.com/_/rust

FROM rust:latest as builder

RUN USER=root cargo new --bin rust-docker-web
WORKDIR ./rust-docker-web
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs && rm ./target/release/deps/pfp_server*

ADD . ./

RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/pfp_server /usr/local/bin/pfp_server

EXPOSE 8080
CMD ["pfp_server"]
