%% -*- mode: prolog -*-

anagram(Word, Candidates, Anagrams) :-
    include(call(anagram_of, Word), Candidates, Anagrams).

anagram_of(X, Y) :-
    string_lower(X, XLow),
    string_lower(Y, YLow),
    XLow \== YLow,
    string_to_list(XLow, XList),
    string_to_list(YLow, YList),
    msort(XList, XSorted),
    msort(YList, YSorted),
    XSorted == YSorted.
