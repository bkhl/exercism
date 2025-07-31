#!/bin/bash

main () {
    input="$1"
    output=""

    while [[ $input =~ ( *)([[:alpha:]])([[:alpha:]]|\')*(.*) ]]; do
        output+="${BASH_REMATCH[2]^}"
        input="${BASH_REMATCH[4]}"
    done

    echo "$output"
}

main "$@"
