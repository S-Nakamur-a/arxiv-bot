FROM rust:1.41

RUN apt-get update -y && apt-get upgrade -y
RUN cargo install diesel_cli --no-default-features --features sqlite

WORKDIR /app
ARG USER
ARG CARGO_TARGET_DIR
RUN USER=$USER cargo init --bin
ENV CARGO_TARGET_DIR=$CARGO_TARGET_DIR

COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock

RUN cargo build --release \
    && rm src/*.rs
