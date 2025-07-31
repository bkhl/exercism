import string
from itertools import islice

CIPHER = {
    k: v for k, v in zip(string.ascii_lowercase, reversed(string.ascii_lowercase))
}


def transpose(c):
    if c in CIPHER:
        return CIPHER[c]
    elif c in string.digits:
        return c
    else:
        return None


def chunk(l, n):
    i = iter(l)
    while True:
        c = list(islice(i, n))
        if c:
            yield c
        else:
            raise StopIteration


def encode(plain_text):
    return " ".join(
        "".join(block)
        for block in chunk(
            (
                optional_c
                for optional_c in (transpose(c.lower()) for c in plain_text)
                if optional_c is not None
            ),
            5,
        )
    )


def decode(ciphered_text):
    return "".join(
        optional_c
        for optional_c in (transpose(c) for c in ciphered_text)
        if optional_c is not None
    )
