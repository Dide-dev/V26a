#
# This program has been developed by students from the bachelor Computer Science at
# Utrecht University within the Software Project course.
# © Copyright Utrecht University (Department of Information and Computing Sciences)
#
set -xe

cd ./backend/
export RUSTFLAGS="-D warnings"
export RUSTDOCFLAGS="-D warnings"
cargo fmt
cargo clippy --all-targets
cargo doc --no-deps
cargo sqlx prepare
cargo build
cargo test

cd ../frontend/
npm i
npm run format
npm run build
npm run check
npm run lint
CI=true npm run test
