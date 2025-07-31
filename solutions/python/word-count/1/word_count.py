import re
import string
from collections import defaultdict

PUNCTUATION = "".join(c for c in string.punctuation if c != "'")


def word_count(phrase):
    result = defaultdict(lambda: 0)

    for word in re.split(f"[\s{PUNCTUATION}]", phrase):
        if word == "":
            continue

        word = re.sub(r"^('+)(.+)\1$", r"\2", word)

        result[word.lower()] += 1

    return result
