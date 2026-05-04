[This program has been developed by students from the bachelor Computer Science at
Utrecht University within the Software Project course.
© Copyright Utrecht University (Department of Information and Computing Sciences)]: #

# Heuristic Trees

## Description

This project creates a free-to-use and open-source [website](https://boom.its.uu.nl) for creating and using _heuristic
trees_.

A _heuristic tree_ is a set of flash cards arranged in a tree-shape. It can help students solve problems, without
giving away the answer, by providing them with _heuristics_. The website can be used by teachers and authors of study
material to create their own trees, which can help their students tackle specific problems. Students are then able to
explore these trees to help them with their exercises.

More on the research behind the use of _heuristic trees_ to help students solve problems can be
read [here](https://doi.org/10.1007/s40751-022-00101-6).

## Developer Setup

The project consists of both a backend and a frontend, which can be found in their respective directories.
Both need to be correctly installed and configured to get the project running.

1. Create a `.env` file by copying the `.env.example` file.
   The variables can be edited to change the database setup or any other environment variables.

2. Install the dependencies for the frontend. For this, you need to have node.js v24 installed.
   In the `frontend` directory, run `npm install`.

3. Setup and run the database. For this, you need to have Docker installed.
   In the directory containing `compose.yml`, run `docker compose up`.

4. Compile and run the backend. For this, you need to have Rust installed.
   First install the `sqlx` utility command, by running
   `cargo install sqlx-cli --no-default-features --features postgres`.
   Go to the `backend` directory.
   While the database is running, run `cargo sqlx migrate run` to initialize the database tables.

## Usage

After installation, to get the website running locally, all you have to do are the following two steps:

1. Turn off any remaining instances that might still be running using `docker compose --profile "*" down`.

2. Run the application in either production or developer mode, using `docker compose --profile prod|dev up --build`.
   Replace prod|dev according to the mode you would like the application to run in.

The website can now be visited locally in a web browser on http://localhost/.

## Features

_The project is still in very active development, so this list is constantly changing, and might be a bit outdated._

* Viewing and exploring published heuristic trees.
* Creating and publishing new heuristic trees.

## Tech Stack

### Frontend

On the frontend, TypeScript is used with the [Svelte](https://svelte.dev) framework. For CSS formatting, Tailwind is
used, and for localization, Paraglide is used.

### Backend

On the backend, Rust is used. For running the webserver, Axum is used.

### Database

A [PostgreSQL](https://postgresql.org) database is used. For communication between the backend and the database, SQLx
is used.

### Testing

For component testing on the frontend, the vitest library is used.
Unit and integration tests in the backend are
written using the built-in testing functionality of cargo, combined with the axum-testing library.

## Contributing

To run the tests in the frontend, run `npm run test` in the `frontend` directory.
To run the tests in the backend, run `cargo test` in the `backend` directory.

When working on the frontend, it is advised to run the application in `dev` mode. This allows you to
enable "watch", which will automatically update the frontend when changes to source files are found.

To enable watch, either use the displayed hotkey within the container, or start the container with `--watch` at the end.

If any new queries were added in the backend, then run `cargo sqlx prepare` in the `backend` directory,
before making a pull request.
This will create files in the `backend/.sqlx` directory, which contain the types for the new query.

## License

The project is distributed under the GNU General Public License (GPLv3), which can be read [here](license.txt).

## Acknowledgements

This project was commissioned by Rogier Bos & Theo van den Bogaart, and has been made possible with the support of
Roxana Radulescu, Guido Mintjes and the people of the _Project Bureau_ (Hieke Keuning, Heysem Kaya & Johan van Rooij).

This program has been developed by students from the bachelor Computer Science at
Utrecht University within the Software Project course.

© Copyright Utrecht University (Department of Information and Computing Sciences)
