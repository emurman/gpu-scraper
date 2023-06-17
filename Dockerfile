FROM rust:1.67 as builder
WORKDIR /usr/src/scraper
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/scraper /usr/local/bin/scraper
CMD ["scraper"]