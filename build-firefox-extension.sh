#!/usr/bin/env bash

set -eo pipefail

XPI_TARGET="${PWD}"/chianti-firefox.xpi
NODE_PACKAGE_MANAGER="pnpm"

if ! command -v "${NODE_PACKAGE_MANAGER}" >/dev/null 2>&1; then
    echo "${NODE_PACKAGE_MANAGER} is not installed"
    echo "Using npm!"
    NODE_PACKAGE_MANAGER="npm"
fi

if ! command -v "7z" >/dev/null 2>&1; then
    echo "7z is not installed"
    exit 1
fi

cd firefox-extension

"${NODE_PACKAGE_MANAGER}" install
"${NODE_PACKAGE_MANAGER}" run build

if [[ -f "${XPI_TARGET}" ]]; then
    rm -v "${XPI_TARGET}"
fi

cd dist
7z a "${XPI_TARGET}" ./*
