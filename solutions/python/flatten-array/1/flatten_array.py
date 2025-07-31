from typing import Sequence


def iterflattened(iterable):
    for element in iterable:
        if element is None:
            continue
        elif isinstance(element, str):
            yield element
        elif isinstance(element, Sequence):
            for sub_element in iterflattened(element):
                yield sub_element
        else:
            yield element


def flatten(iterable):
    return list(iterflattened(iterable))
