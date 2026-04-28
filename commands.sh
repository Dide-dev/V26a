#!/bin/sh
#
# This program has been developed by students from the bachelor Computer Science at
# Utrecht University within the Software Project course.
# © Copyright Utrecht University (Department of Information and Computing Sciences)
#

set -xe

cd ./backend/
cargo fmt
cargo clippy --all-targets -- -D warnings
export RUSTDOCFLAGS="-D warnings"
cargo doc --no-deps
cargo sqlx prepare
cargo build
cargo test

cd ../frontend/
npm install
npm run format
npm run build
npm run check
npm run lint
CI=true npm run test
