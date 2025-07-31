create((X, Y)) :-
    0 =< X, X =< 7,
    0 =< Y, Y =< 7.

attack((X, _),  (X, _)) :- !.
attack((_, Y),  (_, Y)) :- !.
attack((X1, Y1), (X2, Y2)) :- abs(X1-X2) =:= abs(Y1-Y2), !.
