FROM rust:1-alpine AS builder

ARG TARGETARCH
ARG RUST_TARGET

RUN apk add --no-cache \
    build-base \
    file \
    musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs ./
COPY assets ./assets
COPY src ./src
COPY README.md README.zh-CN.md ./

ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN if [ -z "$RUST_TARGET" ]; then \
      case "$TARGETARCH" in \
        amd64) RUST_TARGET="x86_64-unknown-linux-musl" ;; \
        arm64) RUST_TARGET="aarch64-unknown-linux-musl" ;; \
        *) echo "unsupported TARGET ARCH=$TARGETARCH; set --build-arg RUST_TARGET=<rust-musl-target>" >&2; exit 1 ;; \
      esac; \
    fi \
    && rustup target add "$RUST_TARGET" \
    && cargo build --release --locked --target "$RUST_TARGET" \
    && file "target/$RUST_TARGET/release/p2p_clip_bridge_server" | grep -Eqi "statically linked|static-pie linked" \
    && cp "target/$RUST_TARGET/release/p2p_clip_bridge_server" /tmp/p2p_clip_bridge_server

FROM alpine:3.20

LABEL org.opencontainers.image.title="P2P Clip Bridge Server"
LABEL org.opencontainers.image.description="Small P2P WebRTC clipboard and file bridge server"

COPY --from=builder /tmp/p2p_clip_bridge_server /usr/local/bin/p2p_clip_bridge_server

EXPOSE 7259/tcp
EXPOSE 3478/udp

USER nobody:nobody
ENTRYPOINT ["/usr/local/bin/p2p_clip_bridge_server"]
