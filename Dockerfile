FROM rust:buster as run_environment
RUN apt-get update && apt-get install -y \
    curl \
    jq \
    python3 \
    python3-pip \
    python3-setuptools \
    python3-wheel \
    python3-venv libz3-dev libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN pip3 install --upgrade pip
RUN mkdir /bins

FROM run_environment as build_environment
RUN apt-get update && apt-get install -y clang pkg-config cmake \
    && rm -rf /var/lib/apt/lists/*

FROM build_environment as builder
WORKDIR /builder

COPY Cargo.toml .
COPY Cargo.lock .
COPY rust-toolchain.toml .
COPY src ./src
COPY benches ./benches

RUN cargo build --release
RUN cp target/release/gecko /bins/cli_offchain


RUN cargo build --release
RUN cp target/release/gecko /bins/cli_onchain

FROM run_environment
WORKDIR /app
COPY --from=builder /bins /bins

WORKDIR /bins
COPY tests /bins/tests

COPY ui /app/ui
RUN pip3 install -r /app/ui/requirements.txt

EXPOSE 8000

// TODO: Fix for gcc
