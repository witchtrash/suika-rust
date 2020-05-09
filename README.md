# suika

## Setup

1. Copy the `.env.example` file to `.env` and change the connection URL if you need to.
	- If you changed the root password, make sure to change it in the `docker-compose.yml` file as well.
2. Start the project with `docker-compose up --build`
3. The API is accessible at `localhost:8000` and the Postgres server at `localhost:5433`
4. Shell into the `suika-api` container and run `diesel setup` under `/app/suika`. This will create your Postgres database.
	- Create new migrations with `diesel migration generate xxx`
	- Run the migrations with `diesel migration run`
	- Test for rollback errors with `diesel migration redo`

## Hacking
  - The app starts via `cargo watch` so code changes are picked up from the host and the app is reloaded on changes.
