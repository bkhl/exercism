binary(S, N) :-
    string_chars(S, L),
    reverse(L, L2),
    foldl([C, I-N, I2-N2] >> (char_digit(C, D),
                              I2 is I * 2,
                              N2 is N + (I * D)),
          L2, 1-0, _-N).

char_digit('0', 0).
char_digit('1', 1).
