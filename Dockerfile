FROM rust:1.91.0 as build

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y build-essential libc6 && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/app/target/release/axum_course /usr/local/bin/axum_course

WORKDIR /usr/local/bin
CMD ["axum_course"]
