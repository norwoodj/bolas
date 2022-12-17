#!/usr/bin/env bash
set -o errexit; set -o nounset; set -o pipefail

cargo fmt
cargo clippy -- --deny warnings
prettier --write .
