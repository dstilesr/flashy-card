import jinja2
import sqlalchemy as sa
from typing import Optional
from app import models as m
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import HTMLResponse

from .base import BaseController
from ..enums import PartOfSpeech
from ..crud_utils.cards import CardsCRUD


class ListCardsLang(BaseController):
    """
    Controller to list flash cards by language.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            language_slug: str,
            part_of_speech: Optional[PartOfSpeech] = None,
            page: int = 1,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param template_env:
        :param part_of_speech:
        :param language_slug:
        :param page:
        """
        super().__init__(engine, template_env)
        self.language_slug = language_slug
        self.pos = part_of_speech
        self.__crud = CardsCRUD(self.engine)
        self.page = page

    async def get_language(self) -> m.Language:
        """
        Get the language object from the DB.
        :return:
        """
        async with sa_async.AsyncSession(self.engine) as s:
            res = await s.scalars(
                sa.select(m.Language)
                .where(m.Language.slug == self.language_slug)
            )
            lang = res.one_or_none()
        return lang

    async def process_request(self) -> HTMLResponse:
        """
        List flash cards for a given language.
        :return:
        """
        language = await self.get_language()

        filters = {
            "target_language_id": language.id,
            "deleted_at": None
        }
        if self.pos is not None:
            filters.update(part_of_speech=self.pos)
        cards = await self.__crud.list_items(**filters)
        cards, total_pages = self.paginate_list(list(cards), self.page)

        template = self.template_env.get_template("card-list.html.jinja2")
        rsp_body = template.render(
            list_title="List of Cards for %s Language" % language.name,
            cards=cards,
            language=language,
            page=self.page,
            link=f"/cards/{language.slug}/list",
            total_pages=total_pages
        )
        return HTMLResponse(rsp_body)
