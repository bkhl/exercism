#!/bin/bash

# Check if input is an armstrong number. Print "true" or "false" and
# return with success or failure code accordingly.
validate() {
    number="$1"

    sum=0
    digit_count=${#number}
    for (( i = 0; i < digit_count; i++ )); do
        sum=$(( sum + ${number:$i:1} ** digit_count))
    done

    if [[ "$sum" == "$number" ]]; then
        echo true
    else
        echo false
        return 1
    fi
}

validate "$*" || exit 1
