#!/bin/bash

square() {
    n="$1"

    if [[ $n -lt 1 || $n -gt 64 ]]; then
        echo "Error: invalid input"
        return 1
    fi

    bc <<< "2 ^ ($n - 1)"
}

sum_of_squares() {
    n="$1"

    bc <<< "(2 ^ $n) - 1"
}

main() {
    input="$1"

    if [[ $input == total ]]; then
        sum_of_squares 64
    else
        square "$input" || return 1
    fi
}

main "$*" || exit 1
