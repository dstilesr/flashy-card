import jinja2
from typing import Optional
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import RedirectResponse, HTMLResponse

from .base import BaseController
from ..dao.languages import LanguageDAO


class AddLanguagePost(BaseController):
    """
    Controller to add a new language.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param template_env:
        """
        super().__init__(engine, template_env)
        self.crud = LanguageDAO(self.engine)

    async def process_request(self, name: str, notes: str) -> RedirectResponse:
        """
        Add new language to the DB.
        :param name:
        :param notes:
        :return:
        """
        _ = await self.crud.create(
            {"name": name, "notes": notes}
        )
        return RedirectResponse("/languages/list", status_code=302)


class AddLanguagePage(BaseController):
    """
    Render page to add language.
    """

    async def process_request(self, **kwargs) -> HTMLResponse:
        """
        Render page to add language.
        :param kwargs:
        :return:
        """
        template = self.template_env.get_template("language-add.html.jinja2")
        return HTMLResponse(
            template.render()
        )

