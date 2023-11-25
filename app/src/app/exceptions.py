
class BaseAppError(Exception):
    """
    Base exception class.
    """
    pass


class ResourceExists(BaseAppError):
    """
    Thrown when attempting to create a resource that already exists.
    """
    pass


class ResourceNotFound(BaseAppError):
    """
    Thrown when fetching a resource that does not exist.
    """
    pass


class InvalidParameters(BaseAppError):
    """
    Thrown when invalid parameters are sent.
    """
    pass
