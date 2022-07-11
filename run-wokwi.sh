#!/usr/bin/env bash

set -e

if [ "${USER}" == "gitpod" ]; then
    export CURRENT_PROJECT=/workspace/esp-clock
    which idf.py >/dev/null || {
        source /home/gitpod/export-rust.sh > /dev/null 2>&1
        export IDF_TOOLS_PATH=/home/gitpod/.espressif
        source /home/gitpod/.espressif/frameworks/esp-idf-release-v4.4/export.sh
    }
else
    export CURRENT_PROJECT=~/workspace/
fi


#if [ "${USER}" == "gitpod" ];then
#    gp_url=$(gp url 9012)
#    echo "gp_url=${gp_url}"
#    export WOKWI_HOST=${gp_url:8}
#elif [ "${CODESPACE_NAME}" != "" ];then
#    export WOKWI_HOST=${CODESPACE_NAME}-9012.githubpreview.dev
#fi

export ESP_BOARD="esp32c3"
export ESP_ELF="esp-clock"

if [ "${ESP_BOARD}" == "esp32c3" ]; then
    export WOKWI_PROJECT_ID="336529450034266706"
    export ESP_ARCH="riscv32imc-esp-espidf"
    export ESP_BOOTLOADER_OFFSET="0x0000"
    export ESP_PARTITION_TABLE_OFFSET="0x8000"
    export ESP_APP_OFFSET="0x10000"
elif [ "${ESP_BOARD}" == "esp32s2" ]; then
    export WOKWI_PROJECT_ID=""
    export ESP_ARCH="xtensa-esp32s2-espidf"
else
    export WOKWI_PROJECT_ID=""
    export ESP_ARCH="xtensa-esp32-espidf"
fi


cargo espflash save-image app.bin --release 

find target/${ESP_ARCH}/release -name bootloader.bin -exec cp {} . \;
find target/${ESP_ARCH}/release -name partition-table.bin -exec cp {} . \;

python3 esp32-wokwi-gitpod-websocket-server/server.py

#wokwi-server --chip ${ESP_BOARD} --id ${WOKWI_PROJECT_ID} ${CURRENT_PROJECT}/target/${ESP_ARCH}/${BUILD_MODE}/${ESP_ELF} ${ESP_ELF}