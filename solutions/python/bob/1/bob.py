def hey(phrase):
    normalized_phrase = phrase.strip()
    yelling = normalized_phrase.isupper()
    question = normalized_phrase.endswith("?")

    if normalized_phrase:
        if question:
            if yelling:
                return "Calm down, I know what I'm doing!"
            else:
                return "Sure."
        else:
            if yelling:
                return "Whoa, chill out!"
            else:
                return "Whatever."
    else:
        return "Fine. Be that way!"
