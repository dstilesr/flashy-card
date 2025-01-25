import jinja2
import sqlalchemy as sa
from typing import Optional
from datetime import datetime
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import RedirectResponse

from .. import models as m
from .. import exceptions as err
from .base import BaseController
from ..dao.decks import DecksDAO


class AddRemoveFromDeck(BaseController):
    """
    Controller to add or remove a card from a deck.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            deck_id: int,
            card_id: int,
            remove: bool = False,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param deck_id:
        :param card_id:
        :param remove: Whether to remove or add the card.
        :param template_env:
        """
        super().__init__(engine, template_env)
        self.card_id = card_id
        self.deck_id = deck_id
        self.remove = remove
        self.deck_crud = DecksDAO(self.engine)

    async def get_card(self, session: sa_async.AsyncSession) -> m.FlashCard:
        """
        Get the card to add or remove.
        :param session:
        :return:
        """
        res = await session.scalars(
            sa.select(m.FlashCard)
            .where(
                m.FlashCard.deleted_at.is_(None)
                & (m.FlashCard.id == self.card_id)
            )
        )
        card = res.one_or_none()
        self.check_null(card, "Card")
        return card

    async def process_request(self) -> RedirectResponse:
        """
        Add or remove the card from the deck.
        :return: Redirect back to the add / remove page.
        """
        async with sa_async.AsyncSession(self.engine) as session:
            stmt = self.deck_crud.deck_with_cards_stmt(self.deck_id)
            res = await session.scalars(stmt)
            card = await self.get_card(session)
            deck = res.one_or_none()
            self.check_null(deck, "Deck")
            deck_id = self.deck_id

            if self.remove:
                deck.cards.remove(card)
            else:
                deck.cards.append(card)
            deck.updated_at = datetime.utcnow()
            session.add(deck)
            await session.commit()

        back_to = (
            f"/decks/{deck_id:d}/remove-cards" if self.remove
            else f"/decks/{deck_id:d}/add-cards"
        )
        return RedirectResponse(back_to, status_code=302)
