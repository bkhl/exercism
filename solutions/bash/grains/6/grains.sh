#!/bin/bash

# Print the number of grains on the given chess board square.
square() {
    n="$1"

    if [[ $n -lt 1 || $n -gt 64 ]]; then
        echo "Error: invalid input"
        return 1
    fi

    printf %u $((2 ** (n - 1)))
}

# Print the total number of grains on the given chess board square,
# and all the squares before it.
sum_of_squares() {
    n="$1"

    printf %u $(((2 ** n) - 1))
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
