# Flashy Card

## Overview
This a simple web app for studying languages using the "flash card" method. I have
zero frontend experience, so it won't be aesthetically pleasing, but it is good
practice nevertheless.

## Running Locally
In order to run the app locally you must have [Docker](https://www.docker.com/)
installed. In order to spin up the service locally, run `docker compose up --build -d`.
This will spin up two containers:
- A PostgreSQL database to keep the app data.
- A container running the app.

When you do this for the first time, you'll have to set up the DB by installing the
libraries in `local.requirements.txt` and running  `alembic upgrade head`. Once you
have started the app, you can use it by going to `http://localhost:3000/` on your
browser to play around with it. Once you are done, you can take it down by running
`docker compose down`.
