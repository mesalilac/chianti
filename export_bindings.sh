#!/bin/sh

set -e

# Clear old bindings
rm -v ./ts-bindings/bindings/*

cargo test export_bindings

cd ./ts-bindings/

TYPES_EXPORT_FILE="index.d.ts"

printf "// Auto-generated file using export_bindings.sh\n\n" > "${TYPES_EXPORT_FILE}"

for file in $(find ./bindings -name "*.ts"); do
    printf "export * from \'%s\';\n" "${file}" >> "${TYPES_EXPORT_FILE}"
done
