#!/bin/bash

declare -A scores
for score in {A,E,I,O,U,L,N,R,S,T}1 \
             {D,G}2 \
             {B,C,M,P}3 \
             {F,H,V,W,Y}4 \
             K5 \
             {J,X}8 \
             {Q,Z}10; do
    scores[${score:0:1}]="${score:1}"
done

main () {
    word="${1^^}"

    score=0
    for (( i=0; i<${#word}; i++ )); do
        (( score += scores[${word:$i:1}] ))
    done

    echo $score
}

main "$@"
