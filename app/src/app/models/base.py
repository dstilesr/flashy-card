from sqlalchemy.orm import DeclarativeBase
from sqlalchemy.ext.asyncio import AsyncAttrs


class Base(AsyncAttrs, DeclarativeBase):
    """
    Declarative base for ORM.
    """
    pass
