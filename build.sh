#!/usr/bin/env bash

set -e


# Gitpod and VsCode Codespaces tasks do not source the user environment
if [ "${USER}" == "gitpod" ]; then
    which idf.py >/dev/null || {
        source ~/export-esp.sh > /dev/null 2>&1
    }
elif [ "${CODESPACE_NAME}" != "" ]; then
    which idf.py >/dev/null || {
        source ~/export-esp.sh > /dev/null 2>&1
    }
fi


case "$1" in
    ""|"release")
        cargo +esp build --target riscv32imc-esp-espidf --release 
        ;;
    "debug")
        cargo +esp build --target riscv32imc-esp-espidf
        ;;
    *)
        echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
        exit 1;;
esac