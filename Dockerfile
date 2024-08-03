FROM rust:latest

ARG TRUNK_VERSION=v0.16.0
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli \
    && curl -sSfLo - "https://github.com/thedodd/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" | tar -xz -C /usr/local/bin

WORKDIR app
COPY . .
RUN trunk build --release

CMD trunk serve --release --address 0.0.0.0 --port 8080
