anagram(Word, Candidates, Anagrams) :-
    string_to_lowercase_chars(Word, LowerCaseWord),
    msort(LowerCaseWord, SortedWord),
    include(call(anagram_1, LowerCaseWord, SortedWord),
            Candidates,
            Anagrams).

anagram_1(LowerCaseWord, SortedWord, Candidate) :-
    string_to_lowercase_chars(Candidate, LowerCaseCandidate),
    LowerCaseCandidate \== LowerCaseWord,
    msort(LowerCaseCandidate, SortedCandidate),
    SortedCandidate == SortedWord.

string_to_lowercase_chars(Word, LowerCaseChars) :-
    string_chars(Word, Chars),
    maplist(to_lower, Chars, LowerCaseChars).
