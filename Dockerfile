FROM rustlang/rust:nightly AS build-env

RUN apt-get upgrade -y && apt update \
    && apt-get install openssl git ca-certificates musl-tools --no-install-recommends -y \
    && git clone https://github.com/boxpurin/mitamoru.git \
    && cd mitamoru \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo build --release --target=x86_64-unknown-linux-musl

FROM rust:alpine

RUN apk update && apk add ca-certificates openssl && rm -rf /var/cache/apk/*
COPY --from=build-env /mitamoru/target/x86_64-unknown-linux-musl/release/mitamoru /usr/local/bin/mitamoru
ENTRYPOINT ["/usr/local/bin/mitamoru"]