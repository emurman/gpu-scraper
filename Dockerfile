FROM debian:bullseye-slim
COPY target/release/gpu-scraper .
CMD ["gpu-scraper"]