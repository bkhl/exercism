nucleotide_count(S, Counts) :-
    string_chars(S, Cs),
    nucleotide_count_1(Cs, [('A', 0), ('C', 0), ('G', 0), ('T', 0)], Counts).

nucleotide_count_1([], Counts, Counts).

nucleotide_count_1(['A'|Cs], [('A', N), C, G, T], Counts) :-
    N2 is N + 1,
    nucleotide_count_1(Cs, [('A', N2), C, G, T], Counts).

nucleotide_count_1(['C'|Cs], [A, ('C', N), G, T], Counts) :-
    N2 is N + 1,
    nucleotide_count_1(Cs, [A, ('C', N2), G, T], Counts).

nucleotide_count_1(['G'|Cs], [A, C, ('G', N), T], Counts) :-
    N2 is N + 1,
    nucleotide_count_1(Cs, [A, C, ('G', N2), T], Counts).

nucleotide_count_1(['T'|Cs], [A, C, G, ('T', N)], Counts) :-
    N2 is N + 1,
    nucleotide_count_1(Cs, [A, C, G, ('T', N2)], Counts).
