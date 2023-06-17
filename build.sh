#!/bin/bash

docker image rm -f gpu-scraper-app
nohup docker compose build > nohup.txt &