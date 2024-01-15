#!/bin/bash
cargo build -r
cargo test --verbose
sudo mv target/release/semver /usr/bin/
