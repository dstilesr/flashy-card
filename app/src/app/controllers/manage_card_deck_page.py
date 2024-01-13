import jinja2
import sqlalchemy as sa
from sqlalchemy import orm
from typing import Optional, List
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import HTMLResponse

from .. import models as m
from .. import exceptions as err
from .base import BaseController
from ..crud_utils.decks import DecksCRUD


class AddRemoveCardsDeckPage(BaseController):
    """
    Controller to render the page to add or remove cards from a flash deck.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            deck_id: int,
            remove: bool = False,
            page: int = 1,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param deck_id:
        :param remove:
        :param page:
        :param template_env:
        """
        super().__init__(engine, template_env)
        self.deck_id = deck_id
        self.remove = remove
        self.page = page

    async def get_cards_in_deck(self) -> List[m.FlashCard]:
        """
        Get the flash cards present in the deck.
        :return:
        """
        async with sa_async.AsyncSession(self.engine) as session:
            qry = DecksCRUD.deck_with_cards_stmt(self.deck_id)
            res = await session.scalars(qry)
            deck = res.one_or_none()
            if deck is None:
                raise err.ResourceNotFound(
                    "Deck not found!"
                )
            cards = list(deck.cards)

        return cards

    async def process_request(self, **kwargs) -> HTMLResponse:
        """
        Render page to add / remove cards from deck.
        :param kwargs:
        :return:
        """
        deck_crud = DecksCRUD(self.engine)
        deck = await deck_crud.get_one(self.deck_id)
        deck_cards = await self.get_cards_in_deck()
        if self.remove:
            display_cards, total = self.paginate_list(deck_cards, self.page)
            link = f"/decks/{self.deck_id}/remove-cards"
        else:
            exclude = {c.id for c in deck_cards}
            async with sa_async.AsyncSession(self.engine) as sess:
                all_cards = await sess.scalars(
                    sa.select(m.FlashCard)
                    .where(
                        m.FlashCard.deleted_at.is_(None)
                        & m.FlashCard.id.notin_(exclude)
                        & (m.FlashCard.target_language_id == deck.language_id)
                    )
                )
                display_cards, total = self.paginate_list(
                    list(all_cards),
                    self.page
                )
                link = f"/decks/{self.deck_id}/add-cards"

        content = self.template_env\
            .get_template("add-remove-cards.html.jinja2")\
            .render(
                deck_id=deck.id,
                deck_name=deck.name,
                remove=self.remove,
                cards=display_cards,
                link=link,
                page=self.page,
                total_pages=total
            )

        return HTMLResponse(content)
