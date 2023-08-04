#!/usr/bin/env bash
set -o errexit; set -o nounset; set -o pipefail

cargo clippy -- --deny warnings
cargo fmt
prettier --write .
