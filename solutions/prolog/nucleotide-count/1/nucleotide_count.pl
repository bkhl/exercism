nucleotide_count(S, Counts) :-
    string_chars(S, Cs),
    ord_list_to_assoc(['A'-0, 'C'-0, 'G'-0, 'T'-0], Acc),
    nucleotide_count_1(Cs, Acc, CountAssoc),
    assoc_to_list(CountAssoc, CountList),
    maplist(pair_to_tuple, CountList, Counts).

nucleotide_count_1([], CountAssoc, CountAssoc).
nucleotide_count_1([C|Cs], Acc, CountAssoc) :-
    get_assoc(C, Acc, V),
    V2 is V + 1,
    put_assoc(C, Acc, V2, Acc2),
    nucleotide_count_1(Cs, Acc2, CountAssoc).

pair_to_tuple(K-V, (K, V)).
