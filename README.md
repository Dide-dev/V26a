
# Running this application
Create an `.env` file in the root directory.
You can copy the `.env.example`, and possibly edit some values
You can take a look at `.env.example` file to see the required environment variables.

Compile the svelte front end, for this you need nodejs installed.
In the `frontend` directory, run `npm install` and then `npm run build`.

Setup and run the database, for this you need docker installed.
In the root directory, run `docker compose up`.

Compile and run the backend, for this you need rust installed.
First install the `sqlx` utility command, by running `cargo install sqlx-cli`.
Go to the `backend` directory.
While the database is running, run `cargo sqlx migrate run` to initialize the database tables.
Finally run `cargo run`.

# Developing
When working on the front end, the command `npm run dev` in the `frontend` directory can be used.
This will spin up a frontend-only server that automatically reloads pages as source files are saved.

If any new queries were added in the backend, then run `cargo sqlx prepare` in the `backend` directory, before making a pull request.
This will create files in the `backend/.sqlx` directory which contains the types for the new query.


# TODO's
For the backbone:
- Backend bindings for the frontend

For later:
- Authentication
- 404 page, rate limiting, compresssion, loading directory in memory?
- browser side caching
- Database: smaller timeout and keep trying to reconnect
- ...
