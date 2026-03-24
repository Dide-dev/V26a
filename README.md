[This program has been developed by students from the bachelor Computer Science at
Utrecht University within the Software Project course.
© Copyright Utrecht University (Department of Information and Computing Sciences)]: #

# Running this application
Create an `.env` file by copying the `.env.example` file.
The variables can be edited for a different database setup.

Compile the svelte front end, for this you need nodejs installed.
In the `frontend` directory, run `npm install` and then `npm run build`.

Setup and run the database, for this you need docker installed.
In the directory containing `docker-compose.yml`, run `docker compose up`.

Compile and run the backend, for this you need rust installed.
First install the `sqlx` utility command, by running `cargo install sqlx-cli --no-default-features --features postgres`.
Go to the `backend` directory.
While the database is running, run `cargo sqlx migrate run` to initialize the database tables.
Finally run `cargo run`.
The website can now be visited locally in a web browser on `http://localhost:3000/`.

# Developing
When working on the front end, the command `npm run dev` in the `frontend` directory can be used.
This will spin up a frontend-only server that automatically reloads pages as source files are saved.

If any new queries were added in the backend, then run `cargo sqlx prepare` in the `backend` directory, before making a pull request.
This will create files in the `backend/.sqlx` directory which contains the types for the new query.


# TODO's
Some technical TODO's for later:
- Authentication
- 404 page, rate limiting, compresssion, loading directory in memory?
- browser side caching
- Database: smaller timeout and keep trying to reconnect
- ...
