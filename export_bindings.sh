#!/bin/sh

set -e

# cargo install --git https://github.com/Aleph-Alpha/ts-rs --branch feat/cli ts-rs-cli

EXPORT_TARGET="./ts-bindings"

[ -d "${EXPORT_TARGET}" ] && echo "Removing old ${EXPORT_TARGET}" && rm -r "${EXPORT_TARGET}"

echo "Exporting bindings to ${EXPORT_TARGET}"
ts-rs export --index --output-directory "${EXPORT_TARGET}"
