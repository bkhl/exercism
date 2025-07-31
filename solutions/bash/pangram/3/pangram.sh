#!/bin/bash

main () {
    printf -v alphabet '%s' {a..z}
    [[ -z "${alphabet//[${1,,}]/}" ]] && echo true || echo false
}

main "$@"
