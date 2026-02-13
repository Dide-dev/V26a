
# Running this application
First compile the svelte front end.
In the `front_end` directory, run `npm install` and then `npm run build`.
For this you need nodejs installed.

Then compile & run the backend.
In the `back_end` directory, run `cargo run`.
For this you need rust installed.


# TODO's
For the backbone:
- Postgresql setup, comminucating with backend
- instructions for `.env` file creation
- Some kind of OpenAPI specification for backend, to create bindings for frontend
- backend versioning api endpoints
- investigating possible instant reloading with sveltekit

For later:
- Authentication
- 404 page, rate limiting, compresssion, loading directory in memory?
- browser side caching
- ...
