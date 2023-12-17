import jinja2
import sqlalchemy as sa
from typing import Optional, List
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import HTMLResponse

from .. import models as m
from ..enums import PartOfSpeech
from .base import BaseController


class EditCardPage(BaseController):
    """
    Render the page to create / edit a flash card.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            card_id: Optional[int] = None,
            language_id: Optional[int] = None,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param card_id:
        :param language_id:
        :param template_env:
        """
        super().__init__(engine, template_env)
        self.card_id = card_id
        self.language_id = language_id

    async def get_card(self) -> Optional[m.FlashCard]:
        """
        Get the card from the DB (if an ID is given).
        :return:
        """
        if self.card_id is None:
            return None

        async with sa_async.AsyncSession(self.engine) as s:
            res = await s.scalars(
                sa.select(m.FlashCard).where(m.FlashCard.id == self.card_id)
            )
            return res.one()

    async def get_languages(self) -> List[dict]:
        """
        Get the language list from the DB.
        :return:
        """
        langs = []
        async with sa_async.AsyncSession(self.engine) as s:
            res = await s.stream_scalars(sa.select(m.Language))
            async for lang in res:
                langs.append({
                    "id": lang.id,
                    "name": lang.name
                })
        return langs

    async def process_request(self) -> HTMLResponse:
        """
        Render the page.
        :return:
        """
        card = await self.get_card()
        template = self.template_env.get_template("card-edit.html.jinja2")
        template_args = {
            "parts_of_speech": list(PartOfSpeech.__members__.keys()),
            "language_id": None
        }

        if card is not None:
            template_args.update(
                card={
                    "id": card.id,
                    "word": card.target_word,
                    "part_of_speech": card.part_of_speech,
                    "translation": card.translation,
                    "sentence": card.sentence
                },
                language_id=card.target_language_id
            )
        elif self.language_id is not None:
            template_args.update(language_id=self.language_id)
        else:
            languages = await self.get_languages()
            template_args.update(languages=languages)

        content = template.render(**template_args)
        return HTMLResponse(content)
