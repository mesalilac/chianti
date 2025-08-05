#!/bin/sh

set -e

# cargo install --git https://github.com/Aleph-Alpha/ts-rs --branch feat/cli ts-rs-cli

ts-rs export --index
