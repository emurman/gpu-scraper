### gpu scraper

Start DB and server with `docker compose up -d`

Run the scraper with `docker compose run scraper`

Add it to your crontab, example: 

```0 16 * * * docker compose -f $HOME/gpu-scraper/docker-compose.yml run scraper > $HOME/cron/gpu-scraper/`date +\%Y\%m\%d\%H\%M\%S`.log 2>&1```

That's it.

The compile step in the docker build takes an eternity on a Raspberry Pi, about 20 minutes
