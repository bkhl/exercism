triangle(X, Y, Z, T) :-
    msort([X, Y, Z], [A, B, C]),
    A > 0,
    A + B >= C,
    sorted_triangle(A, B, C, T).

sorted_triangle(X, X, X, "equilateral").

sorted_triangle(X, X, _, "isosceles").
sorted_triangle(_, X, X, "isosceles").

sorted_triangle(X, Y, Z, "scalene") :- X =\= Y, X =\= Z.
