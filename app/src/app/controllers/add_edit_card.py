import jinja2
import sqlalchemy as sa
from typing import Optional
from app import models as m
from datetime import datetime
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import RedirectResponse

from .base import BaseController
from ..enums import PartOfSpeech


class EditCard(BaseController):
    """
    Controller to Edit or Create flash cards.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            word: str,
            translation: str,
            language_id: int,
            part_of_speech: PartOfSpeech,
            card_id: Optional[int] = None,
            sentence: Optional[str] = None,
            grammar_info: Optional[str] = None,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param word:
        :param translation:
        :param language_id:
        :param part_of_speech:
        :param card_id:
        :param sentence:
        :param grammar_info:
        :param template_env:
        """
        super().__init__(engine, template_env)
        self.is_create = card_id is None
        self.card_id = card_id
        self.args = {
            "target_word": word,
            "translation": translation,
            "target_language_id": language_id,
            "sentence": sentence,
            "part_of_speech": part_of_speech,
            "grammar_info": grammar_info
        }
        if not self.is_create:
            self.args.update(
                id=card_id,
                updated_at=datetime.utcnow()
            )

    async def process_request(self) -> RedirectResponse:
        """
        Process request to add or edit card.
        :return:
        """
        async with sa_async.AsyncSession(self.engine) as s:
            if self.is_create:
                new_card = m.FlashCard(**self.args)
                s.add(new_card)
            else:
                await s.execute(
                    sa.update(m.FlashCard)
                    .where(m.FlashCard.id == self.card_id)
                    .values(**self.args)
                )
            await s.commit()
            res = await s.scalars(
                sa.select(m.Language)
                .where(m.Language.id == self.args["target_language_id"])
            )
            lang = res.one()

        return RedirectResponse(
            f"/cards/{lang.slug}/list",
            status_code=302
        )
