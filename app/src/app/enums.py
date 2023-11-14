from enum import StrEnum


class PartOfSpeech(StrEnum):
    """
    Parts of speech (types of words) for flash cards.
    """
    NOUN = "noun"
    VERB = "verb"
    CONJUNCTION = "conjunction"
    PREPOSITION = "preposition"
    ADVERB = "adverb"
