#!/bin/bash

main () {
    sentence="${1,,}"

    for letter in {a..z}; do
        if [[ $sentence  != *${letter}* ]]; then
            echo false
            return
        fi
    done

    echo true
}

main "$@"
