binary(S, N) :-
    string_chars(S, L),
    reverse(L, L2),
    foldl([D, I-N, I2-N2] >> (
              ( D == '0' -> N2 is N
              ; D == '1' -> N2 is N + I ),
              I2 is I * 2),
          L2, 1-0, _-N).
