isogram(String) :-
    string_lower(String, LowercaseString),
    string_to_list(LowercaseString, List),
    include(alphabetic, List, LetterBag),
    sort(LetterBag, LetterSet),
    length(LetterBag, X),
    length(LetterSet, X).

alphabetic(C) :- char_type(C, alpha).
