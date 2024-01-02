import sqlalchemy as sa
from datetime import datetime
from typing import Dict, Any, Sequence, Optional

from .base import BaseCrud
from .. import models as m
from .. import exceptions as err


class DecksCRUD(BaseCrud):
    """
    Handler for Card Deck model crud.
    """

    async def get_one(self, deck_id: int) -> m.CardDeck:
        """
        Get a flash card deck.
        :return:
        """
        async with self.get_session() as session:
            res = await session.scalars(
                sa.select(m.CardDeck)
                .where(
                    (m.CardDeck.id == deck_id)
                    & m.CardDeck.deleted_at.is_(None)
                )
            )
            deck = res.one_or_none()
            if deck is None:
                raise err.ResourceNotFound("Deck not found!")

        return deck

    async def list_items(
            self,
            language_id: Optional[int] = None,
            language_slug: Optional[str] = None) -> Sequence[m.CardDeck]:
        """
        List card decks.
        :param language_id:
        :param language_slug:
        :return:
        """
        stmt = sa.select(m.CardDeck)
        if language_id is not None:
            stmt = stmt.where(
                m.CardDeck.deleted_at.is_(None)
                & (m.CardDeck.language_id == language_id)
            )
        elif language_slug is not None:
            stmt = stmt.join(m.CardDeck.language).where(
                m.CardDeck.deleted_at.is_(None)
                & (m.Language.slug == language_slug)
            )
        else:
            stmt = stmt.where(m.CardDeck.deleted_at.is_(None))

        async with self.get_session() as session:
            results = await session.scalars(stmt)
        return results

    async def create(
            self,
            parameters: Dict[str, Any],
            refresh: bool = False) -> m.CardDeck:
        """
        Create new card deck (Does not attach flash cards).
        :param parameters:
        :param refresh:
        :return:
        """
        async with self.get_session() as session:
            deck = m.CardDeck(**parameters)
            session.add(deck)
            await session.commit()
            if refresh:
                await session.refresh(deck)
        return deck

    async def delete(self, deck_id: int):
        """
        Delete a card deck.
        :param deck_id:
        :return:
        """
        async with self.get_session() as session:
            res = await session.execute(
                sa.update(m.CardDeck)
                .where(
                    (m.CardDeck.id == deck_id)
                    & m.CardDeck.deleted_at.is_(None)
                )
                .values(deleted_at=datetime.utcnow())
            )
            if res.rowcount == 0:
                raise err.ResourceNotFound("Found no card deck to delete!")
