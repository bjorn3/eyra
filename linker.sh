#!/usr/bin/bash

args=()

for arg in "$@"; do
    if [[ "$arg" = "-Wl,--version-script"*  ]]; then
    echo ignore
    else
    args+=("$arg")
    fi
done

gcc "${args[@]}"
