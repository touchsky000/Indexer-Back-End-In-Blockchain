FROM rust:1.85-alpine  as builder

WORKDIR ./

COPY Cargo.toml Cargo.lock ./

RUN apk add --no-cache musl-dev
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .

EXPOSE 8080

CMD ["cargo", "run"]