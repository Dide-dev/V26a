This pull request resolves [issue](link-to-issue).

### Description

Mention what functionality/changes have been implemented, how this was done, what relevant technical choices/tradeoffs have been made, etc.
Also specify whether the entire story has been implemented or only certain parts, and if so, which.

### How to test

Describe how the changes can be tested by a reviewer.
For frontend changes, this might include what pages/functionality have been added/changed, and how these are supposed to work from a user's perspective.
For backend changes, this might include a list of API endpoints that has been added/changed, how they are supposed to work, and example inputs.

### Checklist

- [ ] Functionality has been manually tested
- [ ] Unit/component tests have been written where relevant
- [ ] Code has been commented and relevant public functions/types are documented
- [ ] Frontend is correctly formatted and does not produce eslint warnings (run `npx prettier -w .`)
- [ ] Frontend has no failing tests (run `npm run test` and `npx playwright test`)
- [ ] Backend is correctly formatted and does not produce clippy warnings (run `cargo fmt` and `cargo clippy`)
- [ ] Backend has no failing tests (run `cargo test`)
- [ ] Backend sqlx queries are up-to-date (`cargo sqlx prepare` has been run if necessary)
- [ ] Branch is up-to-date with main
- [ ] Story has been moved to 'Review' in YouTrack
