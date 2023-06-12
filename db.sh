#/bin/bash
docker run --name scraper-db -e POSTGRES_PASSWORD=supersecret -d --publish 127.0.0.1:5432:5432 postgres:15