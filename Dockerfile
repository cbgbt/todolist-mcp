FROM rust:1.88-alpine AS builder

RUN apk add --no-cache musl-dev zig && \
    cargo install --locked cargo-zigbuild

RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo zigbuild --release \
    --target x86_64-unknown-linux-musl \
    --target aarch64-unknown-linux-musl

RUN mkdir -p /app/linux && \
    cp target/x86_64-unknown-linux-musl/release/todolist-mcp /app/linux/amd64 && \
    cp target/aarch64-unknown-linux-musl/release/todolist-mcp /app/linux/arm64

FROM scratch

ARG TARGETPLATFORM

COPY --from=builder /app/${TARGETPLATFORM} /todolist-mcp

ENTRYPOINT ["/todolist-mcp"]
