#!/bin/bash

main () {
    sentence="${1//[[:space:]]/}"

    if [[ -z $sentence ]]; then
        echo 'Fine. Be that way!'
    elif [[ $sentence == *\? ]]; then
        if [[ $sentence =~ [[:upper:]] && ! $sentence =~ [[:lower:]] ]]; then
            echo "Calm down, I know what I'm doing!"
        else
            echo 'Sure.'
        fi
    elif [[ $sentence =~ [[:upper:]] && ! $sentence =~ [[:lower:]] ]]; then
        echo 'Whoa, chill out!'
    else
        echo 'Whatever.'
    fi
}

main "$@"
