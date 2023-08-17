#!/usr/bin/env bash
set -e

SCRIPT_DIR=$(realpath $(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd))

WASMEDGE_OPENCV_WIT=${SCRIPT_DIR}/../wasmedge_opencvmini.wit
GENERATED_RS=${SCRIPT_DIR}/../src/generated.rs

if [[ -z "${WITC}" ]]; then
    WITC=witc
fi

if [[ -z "${RUST_FMT}" ]]; then
    RUST_FMT=rustfmt
fi

# Generate the rust code.
${WITC} plugin ${WASMEDGE_OPENCV_WIT} > ${GENERATED_RS}

# Format the rust code.
${RUST_FMT} ${GENERATED_RS}

