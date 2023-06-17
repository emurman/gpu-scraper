#!/bin/bash

cargo build --release
docker build https://github.com/emurman/gpu-scraper.git -t latest