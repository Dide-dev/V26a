::
:: This program has been developed by students from the bachelor Computer Science at
:: Utrecht University within the Software Project course.
:: © Copyright Utrecht University (Department of Information and Computing Sciences)
::

cd ./backend/ || exit
cargo fmt
cargo clippy --all-targets
cargo doc --no-deps
cargo sqlx prepare
cargo build
cargo test

cd ../frontend/ || exit
call npm i
call npx prettier -w ./
call npm run build
call npm run test
