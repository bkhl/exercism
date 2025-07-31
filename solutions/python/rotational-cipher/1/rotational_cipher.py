import string


def transpose(character, key):
    if character not in string.ascii_letters:
        return character

    code = ord(character) + key

    if (character in string.ascii_lowercase and code > ord("z")) or (
        character in string.ascii_uppercase and code > ord("Z")
    ):
        code -= 26

    return chr(code)


def rotate(text, key):
    return "".join(transpose(c, key) for c in text)
