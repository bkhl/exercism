#!/usr/bin/env python3

import re
from collections import Counter

WORD_RE = re.compile(
    r"""
        [a-z](?:[a-z']*[a-z])
        |
        [0-9]+
    """,
    re.VERBOSE,
)


def word_count(phrase):
    return Counter(WORD_RE.findall(phrase.lower()))
