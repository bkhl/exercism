BRACKETS = {"(": ")", "[": "]", "{": "}"}


def is_paired(input_string):
    stack = []

    for c in input_string:
        if c in BRACKETS:
            stack.append(BRACKETS[c])
        elif stack and c == stack[-1]:
            stack.pop()
        elif c in BRACKETS.values():
            return False

    return len(stack) == 0
