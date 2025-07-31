pascal(N, T) :- pascal(N, [[]], T).

pascal(0, T, T2) :- !, reverse(T, [_|T2]).

pascal(N, [R|T], T2) :-
    N2 is N - 1,
    next(R, R2),
    pascal(N2, [R2,R|T], T2).

next([], [1]) :- !.

next(R, R2) :-
    L = [0|R],
    append(R, [0], L2),
    foldl([X, Y, A, [N|A]] >> (N is X + Y),
          L, L2, [], R2).
