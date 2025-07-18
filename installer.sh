#!/usr/bin/env bash

set -eo pipefail

if [[ -n "${XDG_DATA_HOME}" ]]; then
    CHIANTI_DIR="${XDG_DATA_HOME}/chianti"
else
    CHIANTI_DIR="/usr/share/chianti"
fi

NODE_PACKAGE_MANAGER="pnpm"

if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo is not installed"
    exit 1
fi

if ! command -v "${NODE_PACKAGE_MANAGER}" >/dev/null 2>&1; then
    echo "${NODE_PACKAGE_MANAGER} is not installed"
    echo "Using npm!"
    NODE_PACKAGE_MANAGER="npm"
fi

if [[ "$1" == "install" ]];then
    mkdir -pv "${CHIANTI_DIR}"
    echo "Installing chianti server"
    cargo install --path .
    cd web
    "${NODE_PACKAGE_MANAGER}" install
    "${NODE_PACKAGE_MANAGER}" run build
    cp -rv dist "${CHIANTI_DIR}/frontend"
elif [[ "$1" == "uninstall" ]]; then
    echo "Uninstalling chianti server"
    cargo uninstall chianti
    echo "Removing chianti directory"
    rm -rvf "${CHIANTI_DIR}/frontend"
else
    echo "Usage: $0 [install|uninstall]"
fi

