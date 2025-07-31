#!/usr/bin/env bash

main() {
    local input="$1"
    local output

    (( input % 3 == 0 )) && output+=Pling
    (( input % 5 == 0 )) && output+=Plang
    (( input % 7 == 0 )) && output+=Plong

    [[ -z $output ]] && output="$input"

    echo "$output"
}

main "$@"
