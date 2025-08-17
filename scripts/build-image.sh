#!/usr/bin/env bash

set -euo pipefail

if [[ ! -d "./scripts/" ]]; then
    echo "ERROR: Script running in the wrong directory '(${PWD})'"
    exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo is not installed"
    exit 1
fi

if ! command -v docker >/dev/null 2>&1; then
    echo "Docker is not installed"
    exit 1
fi

if ! command -v awk >/dev/null 2>&1; then
    echo "Awk is not installed"
    exit 1
fi

PROJECT_NAME="$(cargo pkgid | awk -F'[/|#]' '{print $(NF-1)}')"
PROJECT_VERSION="$(cargo pkgid | awk -F'[#@]' '{print $NF}')"

if [[ -z "${PROJECT_NAME}" ]]; then
    echo "Unable to determine project name"
    exit 1
fi

if [[ -z "${PROJECT_VERSION}" ]]; then
    echo "Unable to determine project version"
    exit 1
fi

IMAGE_TAG="mesalilac/${PROJECT_NAME}:${PROJECT_VERSION}"

if [[ "$1" = "clean" ]]; then
    docker rmi -f "${IMAGE_TAG}" || true
fi

if [[ -z "$(docker images -q "${IMAGE_TAG}" 2> /dev/null || true)" ]]; then
    docker build -t "${IMAGE_TAG}" .
else
    echo "Docker image with this tag already exists"
    exit 1
fi
