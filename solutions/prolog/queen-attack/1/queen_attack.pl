create((X, Y)) :-
    0=<X,
    X=<7,
    0=<Y,
    Y=<7.

attack((X1, Y1),  (X2, Y2)) :-
    (   (   X1=X2,
            !
        ;   Y1=Y2,
            !
        )
    ;   D=X1-X2,
        D=Y1-X2,
        !
    ).
