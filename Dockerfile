FROM rust:1.64 as builder

WORKDIR /tmp/build
COPY . .

RUN cargo build --release -- -C target-cpu=native

FROM ubuntu:22.04
RUN apt-get update \
    && apt-get -y install libssl-dev \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

COPY --from=builder /tmp/build/target/release/rust_docker_test /usr/src/app/rust_docker_test
COPY --from=builder /tmp/build/assets /usr/src/app/assets

WORKDIR /usr/src/app

CMD ["./rust_docker_test"]
