import jinja2
import sqlalchemy as sa
from sqlalchemy import orm
from typing import Optional
from app import models as m
from datetime import datetime
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import HTMLResponse

from .base import BaseController
from .. import exceptions as err
from ..dao.decks import DecksDAO


class DeckDetail(BaseController):
    """
    Controller to render the deck details page
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            deck_id: int,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param deck_id:
        :param template_env:
        """
        super().__init__(engine, template_env)
        self.deck_id = deck_id

    async def process_request(self) -> HTMLResponse:
        """
        Render the page.
        :return:
        """
        async with sa_async.AsyncSession(self.engine) as session:
            stmt = DecksDAO.deck_with_cards_stmt(self.deck_id)\
                .options(orm.subqueryload(m.CardDeck.language))
            res = await session.scalars(stmt)
            deck = res.one_or_none()
            self.check_null(deck, "Deck")

            content = self.template_env\
                .get_template("deck-detail.html.jinja2")\
                .render(deck=deck)

        return HTMLResponse(content)

