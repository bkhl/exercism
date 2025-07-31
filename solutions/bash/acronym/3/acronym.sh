#!/bin/bash

main () {
    input="$1"
    output=""

    while [[ $input =~ [[:space:]]*([[:alpha:]])([[:alpha:]]|')*(.*) ]]; do
        output+="${BASH_REMATCH[1]^}"
        input="${BASH_REMATCH[3]}"
    done

    echo "$output"
}

main "$@"
