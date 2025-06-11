FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/discord_bot_rust /app/gork-bot

VOLUME ["/app/data"]

CMD ["./gork-bot"]