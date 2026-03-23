*This program has been developed by students from the bachelor Computer Science at*
*Utrecht University within the Software Project course.*
*© Copyright Utrecht University (Department of Information and Computing Sciences)*

# Frontend
For the frontend we have two different kinds of tests: component tests, and end-to-end tests. Both
are written using different frameworks, though can look quite similar.

## Component tests
Component tests are written using the Svelte Testing Library, and can be run with `npm run test`.
They are written in a separate file called `<Component>.test.ts`, or `page.test.ts` if testing a
page.

These work by rendering a component or page in isolation, possibly interacting with it via buttons
or forms, and then verifying the resulting DOM output. For components that make API calls to the
backend, it is necessary to mock the `fetch()` function, since component tests do not have access
to the backend. In these cases care must also be taken when testing the DOM; since API calls will
still happen asynchronously, when trying to find certain API elements you might have to use the
`waitFor` function. This whill automatically retry the given lambda function until it succeeds.

Examples can be found in `src/routes/tree-viewer/NodeViewer.test.ts`,
`src/routes/counter/page.test.ts` and at
https://github.com/testing-library/svelte-testing-library/tree/main/examples.

## End-to-end tests
End-to-end tests are written using the
[Playwright framework](https://playwright.dev/docs/writing-tests), and can be run with
`npx playwright test`. They are written in a separate file in the `tests` directory.

These differ from component tests in that they test more than one component/page, and also have
access to the backend. They are run inside a full browser environment, and can perform any action
the user is capable of, like navigating between pages and pressing buttons. Here you do not need
to worry about timings, since the library will automatically retry actions until they succeed or
are timed out.

Examples can be found in `tests/demo.spec.ts` and the Playwright documentation.

# Backend
For testing the backend we use the built-in testing functionality of cargo, which can be invoked by
running `cargo test`. Since the backend functionally consists mostly of API endpoints, these are
most important to test.
For this, we use the `axum_test` ([documentation](https://docs.rs/axum-test/latest/axum_test/))
library, which allow us to easily test the endpoints by sending requests and inspecting the results.

When writings tests that require a database, we give the test function the `#[sqlx::test]` attribute.
This allows it to take a mock database as parameter, which is automatically set up before the test is
run and cleaned up after it is done. Separate tests using the database therefore *do not* affect each
other.

## Unit tests
Unit tests are written in the same file as the unit they're testing, in a submodule called `test`
with the `#[cfg(test)]` attribute.

These are supposed to test only one API endpoint at a time. If needed, the database can be set up
before making a request by running one or more queries.

Examples can be found in `src/api_v1/demo.rs` and in the `axum_test` documentation.

## Integration tests
Integration tests are written in separate files within the `src/tests` directory. Note that here
each file must start with the `#![cfg(test)]` attribute, and every module must be declared in the
appropriate `mod.rs` file, otherwise the tests will not be run.

These test more than one endpoint at a time, and shouldn't have to access the database directly.
They are meant to ensure that different endpoints work correctly together, for example creating,
getting and updating a tree.

Examples can be found in `src/tests/api_v1/trees.rs`.
