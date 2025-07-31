#!/bin/bash

# Check if input is an armstrong number. Print "true" or "false" and
# return with success or failure code accordingly.
validate() {
    n="$1"

    sum=0
    for (( i = 0; i < ${#n}; i++ )); do
        sum=$(( sum + ${n:$i:1} ** ${#n} ))
    done

    if [[ "$sum" == "$n" ]]; then
        echo true
    else
        echo false
        return 1
    fi
}

validate "$*" || exit 1
