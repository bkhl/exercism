#!/bin/bash

main () {
    printf -v alphabet '%s' {a..z}
    if [[ -z "${alphabet//[${1,,}]/}" ]]; then
        echo true
    else
        echo false
    fi
}

main "$@"
