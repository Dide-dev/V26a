This pull request resolves [issue](link-to-issue).

### Description

Mention what functionality/changes have been implemented, how this was done, what relevant technical choices/tradeoffs
have been made, etc.
Also specify whether the entire story has been implemented or only certain parts, and if so, which.

### How to test

Describe how the changes can be tested by a reviewer.
For frontend changes, this might include what pages/functionality have been added/changed, and how these are supposed to
work from a user's perspective.
For backend changes, this might include a list of API endpoints that has been added/changed, how they are supposed to
work, and example inputs.

### Checklist

- [ ] Functionality has been manually tested
- [ ] Unit/component tests have been written where relevant
- [ ] Integration tests have been written where relevant
- [ ] Code has been commented and relevant public functions/types are documented
- [ ] `utoipa` API-documentation has been added or updated for API-endpoints
- [ ] Files have a UU copyright comment at the top, and a copyright notice on the GUI if applicable
- [ ] Ran commands.bat or commands.sh to make sure that:
  * Backend is correctly formatted and does not produce clippy warnings
  * Backend sqlx queries are up-to-date
  * Backend has no failing tests
  * Frontend is correctly formatted and does not produce TS errors or eslint warnings
  * Frontend has no failing tests
- [ ] Branch is up-to-date with main
- [ ] Story has been moved to 'Review' in YouTrack
