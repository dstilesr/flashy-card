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
    ADJECTIVE = "ADJECTIVE"
    IDIOM = "IDIOM"
    ROOT = "ROOT"


class StudyType(StrEnum):
    """
    Study modes for flash cards.
    """
    from_target = "from_target"
    to_target = "to_target"
