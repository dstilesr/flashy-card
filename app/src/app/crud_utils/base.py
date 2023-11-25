import logging
from abc import ABC, abstractmethod
from typing import Dict, Any, AsyncIterable
from sqlalchemy.ext.asyncio import AsyncSession

from ..models import Base as ModelBase


class BaseCrud(ABC):
    """
    Base CRUD utils class.
    """

    def __init__(self):
        self.logger = logging.getLogger(type(self).__name__)

    @abstractmethod
    async def create(self, session: AsyncSession, parameters: Dict[str, Any]):
        """
        Create a record in the DB.
        :param parameters:
        :param session:
        :return:
        """
        pass

    @abstractmethod
    async def list_items(
            self,
            session: AsyncSession) -> AsyncIterable[ModelBase]:
        """
        List records.
        :param session:
        :return:
        """
        pass

    @abstractmethod
    async def get_one(self, session: AsyncSession) -> ModelBase:
        """
        Get a single item from the DB.
        :param session:
        :return:
        """
        pass
