#!/usr/bin/env python3

import re
import string
from collections import Counter

PUNCTUATION = string.punctuation.replace("'", "")

SPLIT_RE = re.compile(rf"[\s{PUNCTUATION}]+")
QUOTED_WORD_RE = re.compile(r"^'(.+)'$")


def word_count(phrase):
    result = Counter()

    for word in SPLIT_RE.split(phrase.lower()):
        if not word:
            continue

        word = QUOTED_WORD_RE.sub(r"\1", word)

        result[word] += 1

    return result
