::
:: This program has been developed by students from the bachelor Computer Science at
:: Utrecht University within the Software Project course.
:: © Copyright Utrecht University (Department of Information and Computing Sciences)
::

cd ./backend/                               || exit 1
cargo fmt                                   || exit 1
cargo clippy --all-targets -- -D warnings   || exit 1
set RUSTDOCFLAGS=-D warnings
cargo doc --no-deps                         || exit 1
cargo sqlx prepare                          || exit 1
cargo build                                 || exit 1
cargo test                                  || exit 1

cd ../frontend/                             || exit 1
call npm install                            || exit 1
call npm run format                         || exit 1
call npm run build                          || exit 1
call npm run lint                           || exit 1
set CI=true
call npm run test                           || exit 1
