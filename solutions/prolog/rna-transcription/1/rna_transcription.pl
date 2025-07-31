rna_transcription(Rna, Dna) :-
    string_chars(Rna, L),
    maplist(complement, L, L2),
    string_chars(Dna, L2).

complement('A', 'U').
complement('C', 'G').
complement('G', 'C').
complement('T', 'A').
