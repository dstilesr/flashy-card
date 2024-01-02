import jinja2
import sqlalchemy as sa
from typing import Optional
from app import models as m
from datetime import datetime
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import RedirectResponse

from .base import BaseController
from ..crud_utils.decks import DecksCRUD


class EditCardDeck(BaseController):
    """
    Controller to Edit or Create flash card decks.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            name: str,
            context: str,
            language_id: int,
            deck_id: Optional[int] = None,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param name:
        :param context:
        :param language_id:
        :param deck_id:
        :param template_env:
        """
        super().__init__(engine, template_env)
        self.name = name
        self.context = context
        self.language_id = language_id
        self.deck_id = deck_id
        self.deck_crud = DecksCRUD(self.engine)

    async def edit_deck(self):
        """
        Edit the card deck when it exists.
        :return:
        """
        stmt = sa.update(m.CardDeck)\
            .where(m.CardDeck.id == self.deck_id)\
            .values(
                language_id=self.language_id,
                name=self.name,
                context=self.context,
                updated_at=datetime.utcnow()
            )
        async with sa_async.AsyncSession(self.engine) as session:
            result = await session.execute(stmt)
            await session.commit()
            self.logger.warning("Updated: %d" % result.rowcount)

    async def create_deck(self) -> m.CardDeck:
        """
        Make new deck.
        :return:
        """
        deck = await self.deck_crud.create(
            {
                "name": self.name,
                "context": self.context,
                "language_id": self.language_id
            },
            refresh=True
        )
        return deck

    async def process_request(self, **kwargs) -> RedirectResponse:
        """
        Add or edit a card deck.
        :return:
        """
        if self.deck_id is not None:
            await self.edit_deck()
            deck_id = self.deck_id
        else:
            deck = await self.create_deck()
            deck_id = deck.id

        return RedirectResponse(
            f"/decks/{deck_id}/detail",
            status_code=302
        )
