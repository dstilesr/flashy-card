import jinja2
from typing import Optional
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import HTMLResponse

from .base import BaseController
from ..crud_utils.languages import LanguageCRUD


class ListLanguages(BaseController):
    """
    Controller to list languages.
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
        self.crud = LanguageCRUD()

    async def process_request(self) -> HTMLResponse:
        """
        List languages page render.
        :return:
        """
        items = []
        async with sa_async.AsyncSession(self.engine) as session:
            languages = await self.crud.list_items(session)
            async for lang in languages:
                items.append({
                    "name": lang.name,
                    "notes": lang.notes,
                    "slug": lang.slug
                })

        template = self.template_env.get_template("languages.html.jinja2")
        rsp_body = template.render(
            languages=items
        )
        return HTMLResponse(rsp_body)
