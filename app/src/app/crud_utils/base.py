import logging
from abc import ABC, abstractmethod
from typing import Dict, Any, Sequence
import sqlalchemy.ext.asyncio as sa_async

from ..models import Base as ModelBase


class BaseCrud(ABC):
    """
    Base CRUD utils class.
    """

    def __init__(self, engine: sa_async.AsyncEngine):
        self.logger = logging.getLogger(type(self).__name__)
        self.__engine = engine

    @property
    def engine(self) -> sa_async.AsyncEngine:
        """
        SQL Engine.
        """
        return self.__engine

    def get_session(self) -> sa_async.AsyncSession:
        """
        Get a Session for the SQL engine.
        """
        return sa_async.AsyncSession(self.engine)

    @abstractmethod
    async def create(
            self,
            parameters: Dict[str, Any],
            refresh: bool = False) -> ModelBase:
        """
        Create a record in the DB.
        :param parameters:
        :param refresh:
        :return:
        """
        pass

    @abstractmethod
    async def list_items(self) -> Sequence[ModelBase]:
        """
        List records.
        :return:
        """
        pass

    @abstractmethod
    async def get_one(self) -> ModelBase:
        """
        Get a single item from the DB.
        :return:
        """
        pass
