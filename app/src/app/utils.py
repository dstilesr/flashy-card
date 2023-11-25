import re


def to_slug(name: str) -> str:
    """
    Convert a string to slug-like format.
    :param name:
    :return:
    """
    clean = re.sub(r"[^\w]+", "-", name)
    return clean.strip("-").lower()
