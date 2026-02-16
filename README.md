
# Running this application
Create an `.env` file in the root directory.
You can copy the `.env.example`, and possibly edit some values
You can take a look at `.env.example` file to see the required environment variables.

Setup and run the database, for this you need docker installed.
In the root directory, run `docker compose up`.

Compile the svelte front end, for this you need nodejs installed.
In the `frontend` directory, run `npm install` and then `npm run build`.

Compile and run the backend, for this you need rust installed.
In the `backend` directory, run `cargo run`.


# TODO's
For the backbone:
- Backend bindings for the frontend
- investigating possible instant reloading with sveltekit

For later:
- Authentication
- 404 page, rate limiting, compresssion, loading directory in memory?
- browser side caching
- ...
