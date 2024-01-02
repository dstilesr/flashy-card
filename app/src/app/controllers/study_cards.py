import jinja2
import random
import sqlalchemy as sa
from datetime import datetime
from typing import Optional, List
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import HTMLResponse

from .. import models as m
from .base import BaseController
from ..crud_utils.decks import DecksCRUD


class StudyCardPage(BaseController):
    """
    Render the page to study a flash card.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            deck_id: int,
            idx: int = 1,
            show_example: bool = False,
            show_answer: bool = False,
            to_target: bool = False,
            rnd_seed: Optional[int] = None,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param deck_id:
        :param idx:
        :param show_example:
        :param show_answer:
        :param to_target:
        :param rnd_seed:
        :param template_env:
        """
        super().__init__(engine, template_env)
        self.deck_id = deck_id
        self.idx = idx
        self.to_target = to_target
        self.example = show_example
        self.answer = show_answer

        # Set seed to timestamp
        if rnd_seed is None:
            self.rnd_seed = round(datetime.utcnow().timestamp())
        else:
            self.rnd_seed = rnd_seed

    def select_card(self, cards: List[m.FlashCard]) -> m.FlashCard:
        """
        Select the card at the appropriate index from the shuffled list.
        :param cards:
        :return:
        """
        # Pre-sort to ensure consistency
        cards.sort(key=lambda card: (card.part_of_speech, card.updated_at))
        random.seed(self.rnd_seed)
        random.shuffle(cards)
        return cards[self.idx - 1]

    async def process_request(self, **kwargs) -> HTMLResponse:
        """
        Show the page to study a card.
        :param kwargs:
        :return:
        """
        async with sa_async.AsyncSession(self.engine) as session:
            deck_select = DecksCRUD.deck_with_cards_stmt(self.deck_id)
            result = await session.scalars(deck_select)
            deck = result.one_or_none()
            self.check_null(deck, "Deck")

            # Render page
            page = self.template_env\
                .get_template("study-card.html.jinja2")\
                .render(
                    deck=deck,
                    card=self.select_card(list(deck.cards)),
                    rnd_seed=self.rnd_seed,
                    to_target=self.to_target,
                    idx=self.idx,
                    total_cards=len(deck.cards),
                    show_all=self.answer,
                    example=self.example
                )

        return HTMLResponse(page)
