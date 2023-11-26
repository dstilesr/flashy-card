import jinja2
import logging
from typing import Optional
from abc import ABC, abstractmethod
from fastapi.responses import Response
from ..template_util import TEMPLATE_ENV
from sqlalchemy.ext.asyncio import AsyncEngine


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
    async def process_request(self) -> Response:
        """
        Process request and return a response.
        :return:
        """
        pass
