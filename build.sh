#!/bin/bash

set -e


if [ "${USER}" == "gitpod" ]; then
    which idf.py >/dev/null || {
        source /home/gitpod/export-rust.sh > /dev/null 2>&1
        export IDF_TOOLS_PATH=/home/gitpod/.espressif
        source /home/gitpod/.espressif/frameworks/esp-idf-release-v4.4/export.sh
    }
fi

cargo build --release