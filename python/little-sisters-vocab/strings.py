"""Functions for creating, transforming, and adding prefixes to strings."""


def add_prefix_un(word: str) -> str:
    """Take the given word and add the "un" prefix.

    :param word: Root word.
    :return: Root word prepended with "un".
    """

    return "un" + word


def make_word_groups(vocab_words: list) -> str:
    """Transform a list containing a prefix and words into a string with the prefix followed by the words with prefix prepended.

    :param vocab_words: list of vocabulary words with prefix in first index.
    :return: List with prefix in first index followed by vocabulary
    words with prefix applied.

    This function takes a `vocab_words` list and returns a string
    with the prefix and the words with prefix applied, separated
     by " :: ".

    :Example:

    >>> print(list("en", "close", "joy", "lighten"))
    "en :: enclose :: enjoy :: enlighten".
    """

    prefix = vocab_words[0]
    words = vocab_words[1:]
    vocab_prefixed_words = [prefix]

    for word in words:
        vocab_prefixed_words.append(prefix + word)

    return " :: ".join(vocab_prefixed_words)


def remove_suffix_ness(word: str) -> str:
    """Remove the suffix from the word while keeping spelling in mind.

    :param word: Word to remove suffix from.
    :return: Word with suffix removed & spelling adjusted.

    :Example:

    >>> print(remove_suffix_ness("heaviness"))
    "heavy"
    >>> print(remove_suffix_ness)("sadness")
    "sad"
    """

    word_no_suffix = word[:-4]
    if word_no_suffix[-1] == "i":
        word_no_suffix = word_no_suffix[:-1] + "y"

    return word_no_suffix


def adjective_to_verb(sentence: str, index: int) -> str:
    """Change the adjective within the sentence to a verb.

    :param sentence: Sentence containing the word to remove and
    transform.
    :param index: Index of the word to remove and transform.
    :return: Word that changes the extracted adjective to a verb.

    :Example:

    >>> print(adjective_to_verb("It got dark as the sun set.", 2))
    "darken"
    """

    return sentence[:-1].split()[index] + "en"
