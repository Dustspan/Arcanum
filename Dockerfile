# 构建阶段
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src
COPY src ./src
RUN touch src/main.rs && cargo build --release

# 运行阶段
FROM alpine:3.19
RUN apk add --no-cache ca-certificates tzdata
WORKDIR /app
COPY --from=builder /app/target/release/arcanum /app/
RUN mkdir -p /app/data
ENV PORT=3000
ENV DATA_DIR=/app/data
EXPOSE 3000
CMD ["./arcanum"]
