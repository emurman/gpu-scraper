FROM rust:1.67 as builder
WORKDIR /usr/src/scraper
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl1.1 libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/gpu-scraper /usr/local/bin/gpu-scraper
CMD ["gpu-scraper"]