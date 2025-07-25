#!/usr/bin/env bash

set -eo pipefail

EXTENSION_PATH="${PWD}"/browser-extension
FIREFOX_TARGET="${PWD}"/chianti-firefox.xpi
CHROME_TARGET="${PWD}"/chianti-chrome.crx
NODE_PACKAGE_MANAGER="pnpm"

if ! command -v "${NODE_PACKAGE_MANAGER}" >/dev/null 2>&1; then
    echo "${NODE_PACKAGE_MANAGER} is not installed"
    echo "Using npm!"
    NODE_PACKAGE_MANAGER="npm"
fi

if ! command -v "zip" >/dev/null 2>&1; then
    echo "zip is not installed"
    exit 1
fi

cd "${EXTENSION_PATH}"

function build_firefox {
    echo "Building extension for Firefox"
    "${NODE_PACKAGE_MANAGER}" install

    TARGET_BROWSER=firefox "${NODE_PACKAGE_MANAGER}" build
    [[ -f "${FIREFOX_TARGET}" ]] && rm -v "${FIREFOX_TARGET}"

    cd "${EXTENSION_PATH}/firefox-dist"
    zip -r "${FIREFOX_TARGET}" .
}

function build_chrome {
    echo "Building extension for Chrome"
    "${NODE_PACKAGE_MANAGER}" install

    TARGET_BROWSER=chrome "${NODE_PACKAGE_MANAGER}" build
    [[ -f "${CHROME_TARGET}" ]] && rm -v "${CHROME_TARGET}"

    cd "${EXTENSION_PATH}/chrome-dist"
    zip -r "${CHROME_TARGET}" .
}

if [[ "$1" = "firefox" ]];
then
    echo "------------------------------------"
    build_firefox
    echo "------------------------------------"
elif [[ "$1" = "chrome" ]]; then
    echo "------------------------------------"
    build_chrome
    echo "------------------------------------"
elif [[ "$1" = "all" ]]; then
    echo "------------------------------------"
    echo "Building for all browsers"
    build_firefox
    build_chrome
    echo "------------------------------------"
else
    echo "Usage: ./build-browser-extension.sh <all, firefox, chrome>"
    echo "firefox - build extension for firefox"
    echo "chrome  - build extension for chrome"
fi
