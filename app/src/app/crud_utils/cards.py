import sqlalchemy as sa
from datetime import datetime
from typing import Dict, Any, Sequence, Optional

from .base import BaseCrud
from .. import exceptions as err
from ..models import FlashCard, card_deck_assoc


class CardsCRUD(BaseCrud):
    """
    Handler for language model crud.
    """

    async def get_one(self, card_id: int) -> FlashCard:
        """
        Get a single flash card.
        :param card_id:
        :return:
        """
        async with self.get_session() as session:
            res = await session.scalars(
                sa.select(FlashCard).where(FlashCard.id == card_id)
            )
            card = res.one_or_none()
            if card is None:
                raise err.ResourceNotFound("Card not found!")
        return card

    async def create(
            self,
            parameters: Dict[str, Any],
            refresh: bool = False) -> FlashCard:
        """
        Insert card record.
        :param parameters: Dictionary with card attributes.
        :param refresh:
        :return:
        """
        async with self.get_session() as session:
            card = FlashCard(**parameters)
            session.add(card)
            await session.commit()
            if refresh:
                await session.refresh(card)
        return card

    async def list_items(
            self,
            page: Optional[int] = None,
            page_size: Optional[int] = None,
            **filters) -> Sequence[FlashCard]:
        """
        List available cards.
        :param page:
        :param page_size:
        :param filters: Values to filter the cards.
        :return:
        """
        stmt = sa.select(FlashCard)
        condition = sa.true()
        for k, v in filters.items():
            if v is None:
                condition = condition & (getattr(FlashCard, k).is_(None))
            else:
                condition = condition & (getattr(FlashCard, k) == v)

        stmt = stmt.where(condition).order_by(FlashCard.created_at)
        async with self.get_session() as session:
            cards = await session.scalars(stmt)
            return cards

    async def delete(self, card_id: int):
        """
        Delete the given card.
        :param card_id:
        :return:
        """
        async with self.get_session() as session:
            # Delete association to existing decks
            res = await session.execute(
                sa.delete(card_deck_assoc)
                .where(card_deck_assoc.c.card_id == card_id)
            )
            self.logger.info("Deleted from %d decks" % res.rowcount)

            # Delete card
            await session.execute(
                sa.update(FlashCard)
                .where(FlashCard.id == card_id)
                .values(deleted_at=datetime.utcnow())
            )
            await session.commit()
