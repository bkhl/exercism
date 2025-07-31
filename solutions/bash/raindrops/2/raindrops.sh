#!/usr/bin/env bash

main() {
    local input="$1"
    local output

    if (( input % 3 == 0 )); then
        output+=Pling
    fi
    if (( input % 5 == 0 )); then
        output+=Plang
    fi
    if (( input % 7 == 0 )); then
        output+=Plong
    fi

    if [[ -n $output ]]; then
        echo "$output"
    else
        echo "$input"
    fi
}

main "$@"
