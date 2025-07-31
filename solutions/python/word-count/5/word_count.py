#!/usr/bin/env python3

import re
from collections import Counter

WORD_RE = re.compile("(?<![a-z0-9])([a-z0-9][a-z0-9']*[a-z0-9]|[a-z0-9]+)(?![a-z0-9])")


def word_count(phrase):
    return Counter(WORD_RE.findall(phrase.lower()))
