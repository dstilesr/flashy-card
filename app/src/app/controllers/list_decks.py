import jinja2
import sqlalchemy as sa
from sqlalchemy import orm
from typing import Optional
from app import models as m
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import HTMLResponse

from .base import BaseController
from ..crud_utils.decks import DecksCRUD


class ListDecksLang(BaseController):
    """
    Controller to list Card Decks by language.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            language_slug: str,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param template_env:
        :param language_slug:
        """
        super().__init__(engine, template_env)
        self.language_slug = language_slug
        self.__crud = DecksCRUD(self.engine)

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

    async def list_decks(self) -> list[dict]:
        """
        List card decks along with their total cards.
        :return:
        """
        stmt = sa.select(m.CardDeck)\
            .join(m.CardDeck.language)\
            .where(m.Language.slug == self.language_slug)\
            .options(orm.subqueryload(m.CardDeck.cards))
        items = []
        async with sa_async.AsyncSession(self.engine) as sess:
            results = await sess.stream_scalars(stmt)
            async for result in results:
                items.append({
                    "id": result.id,
                    "name": result.name,
                    "context": result.context,
                    "total_cards": len(result.cards)
                })
        return items

    async def process_request(self) -> HTMLResponse:
        """
        List flash cards for a given language.
        :return:
        """
        decks = await self.list_decks()
        language = await self.get_language()
        rsp_body = self.template_env.get_template("list-card-decks.html.jinja2").render(
            decks=decks,
            language_id=language.id
        )
        return HTMLResponse(rsp_body)
