#!/usr/bin/env sh

set -x

cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-unknown-linux-gnu
cross build --release --target x86_64-unknown-linux-gnu

mkdir -p release
cp target/aarch64-apple-darwin/release/uak      release/uak-aarch64-apple-darwin
cp target/x86_64-apple-darwin/release/uak       release/uak-x86_64-apple-darwin
cp target/aarch64-unknown-linux-gnu/release/uak release/uak-aarch64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/uak  release/uak-x86_64-unknown-linux-gnu
