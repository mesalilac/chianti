#!/bin/sh

set -e

if [ -d "./ts-bindings/bindings" ]; then
    echo "Removing old bindings"
    echo "--------------------------"
    rm -rv ./ts-bindings/bindings
    echo "--------------------------"
fi

echo "Exporting bindings"
echo "--------------------------"
cargo test export_bindings
echo "--------------------------"

cd ./ts-bindings/

TYPES_EXPORT_FILE="index.d.ts"

printf "// Auto-generated file using export_bindings.sh\n\n" > "${TYPES_EXPORT_FILE}"

for file in $(find ./bindings -name "*.ts"); do
    printf "export * from \'%s\';\n" "${file}" >> "${TYPES_EXPORT_FILE}"
done
