#!/bin/bash

cargo build --release
docker build . -t scraper