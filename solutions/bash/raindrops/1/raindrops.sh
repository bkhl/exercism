#!/usr/bin/env basho

main() {
    local input="$1"

    local makes_noise

    if (( input % 3 == 0 )); then
        printf Pling
        makes_noise=1
    fi
    if (( input % 5 == 0 )); then
        printf Plang
        makes_noise=1
    fi
    if (( input % 7 == 0 )); then
        printf Plong
        makes_noise=1
    fi

    if (( ! makes_noise )); then
        printf "%s" "$input"
    fi
}

main "$@"
