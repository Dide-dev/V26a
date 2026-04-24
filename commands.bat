cd ./backend/
cargo fmt
cargo clippy --all-targets
cargo doc --no-deps
cargo sqlx prepare
cargo build
cargo test

cd ../frontend/
call npm i
call npx prettier -w ./
call npm run build
call npm run test
