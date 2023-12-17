from enum import StrEnum


class PartOfSpeech(StrEnum):
    """
    Parts of speech (types of words) for flash cards.
    """
    NOUN = "NOUN"
    VERB = "VERB"
    CONJUNCTION = "CONJUNCTION"
    PREPOSITION = "PREPOSITION"
    ADVERB = "ADVERB"
