import jinja2
import sqlalchemy as sa
from typing import Optional
from app import models as m
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import HTMLResponse

from .base import BaseController
from ..enums import PartOfSpeech


class ListCardsLang(BaseController):
    """
    Controller to list flash cards by language.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            language_slug: str,
            part_of_speech: Optional[PartOfSpeech] = None,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param template_env:
        :param part_of_speech:
        :param language_slug:
        """
        super().__init__(engine, template_env)
        self.language_slug = language_slug
        self.pos = part_of_speech

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

    async def process_request(self) -> HTMLResponse:
        """
        List flash cards for a given language.
        :return:
        """
        items = []
        language = await self.get_language()

        filters = (
            (m.FlashCard.target_language_id == language.id)
            & m.FlashCard.deleted_at.is_(None)
        )
        if self.pos is not None:
            filters = filters & (m.FlashCard.part_of_speech == self.pos)
        async with sa_async.AsyncSession(self.engine) as session:
            cards = await session.stream_scalars(
                sa.select(m.FlashCard)
                .where(filters)
                .order_by(m.FlashCard.created_at)
            )
            async for card in cards:
                items.append({
                    "id": card.id,
                    "word": card.target_word,
                    "part_of_speech": card.part_of_speech,
                    "translation": card.translation,
                    "sentence": card.sentence,
                    "target_language_id": card.target_language_id
                })

        template = self.template_env.get_template("card-list.html.jinja2")
        rsp_body = template.render(
            list_title="List of Cards for %s Language" % language.name,
            cards=items,
            language_id=language.id
        )
        return HTMLResponse(rsp_body)
