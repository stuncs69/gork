FROM rust:1.76 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/gork-bot /app/gork-bot

VOLUME ["/app/data"]

CMD ["./gork-bot"]