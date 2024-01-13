import jinja2
import logging
from abc import ABC, abstractmethod
from fastapi.responses import Response
from ..template_util import TEMPLATE_ENV
from sqlalchemy.ext.asyncio import AsyncEngine
from typing import Optional, Tuple, List, TypeVar

from .. import models as m
from .. import exceptions as err
from ..constants import PAGE_SIZE

T = TypeVar("T", bound=m.Base)


class BaseController(ABC):
    """
    Base class for controllers.
    """

    def __init__(
            self,
            engine: AsyncEngine,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param template_env:
        """
        self.logger = logging.getLogger(type(self).__name__)
        self.__engine = engine
        if template_env is None:
            template_env = TEMPLATE_ENV
        self.__template_env = template_env

    @property
    def engine(self) -> AsyncEngine:
        """
        SQL engine.
        """
        return self.__engine

    @property
    def template_env(self) -> jinja2.Environment:
        """
        Template environment.
        """
        return self.__template_env

    @abstractmethod
    async def process_request(self, **kwargs) -> Response:
        """
        Process request and return a response.
        :return:
        """
        pass

    @staticmethod
    def check_null(obj: Optional[m.Base], obj_type: str):
        """
        Check if the object is null and raise an error if it is.
        :param obj:
        :param obj_type:
        :return:
        """
        if obj is None:
            raise err.ResourceNotFound("%s not found!" % obj_type)

    @staticmethod
    def paginate_list(
            items: List[T],
            page: int) -> Tuple[List[T], int]:
        """
        Get the given "page" from a large list of items.
        :param items: List of items.
        :param page: Page to fetch.
        :return: List of items, total pages.
        """
        total = len(items)
        if total == 0:
            return items, 1

        total_pages = (total // PAGE_SIZE) + min(1, total % PAGE_SIZE)
        start = min(((page - 1) * PAGE_SIZE, total - 1))
        return items[start:start + PAGE_SIZE], total_pages
