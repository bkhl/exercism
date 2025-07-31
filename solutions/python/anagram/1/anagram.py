def detect_anagrams(word, candidates):
    word_lower = word.lower()
    word_sorted = sorted(word_lower)
    return [
        candidate
        for candidate, candidate_lower in (
            (candidate, candidate.lower()) for candidate in candidates
        )
        if candidate_lower != word_lower and sorted(candidate_lower) == word_sorted
    ]
