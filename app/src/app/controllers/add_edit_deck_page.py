import jinja2
import sqlalchemy as sa
from typing import Optional, List
import sqlalchemy.ext.asyncio as sa_async
from fastapi.responses import HTMLResponse

from .. import models as m
from ..enums import PartOfSpeech
from .base import BaseController
from ..crud_utils.decks import DecksCRUD
from ..crud_utils.languages import LanguageCRUD


class AddEditDeckPage(BaseController):
    """
    Controller to render the page with the form to add or edit a flash card
    deck.
    """

    def __init__(
            self,
            engine: sa_async.AsyncEngine,
            deck_id: Optional[int] = None,
            language_id: Optional[int] = None,
            template_env: Optional[jinja2.Environment] = None):
        """
        :param engine:
        :param deck_id:
        :param language_id:
        :param template_env:
        """
        super().__init__(engine, template_env)
        self.deck_id = deck_id
        self.language_id = language_id

    async def process_request(self) -> HTMLResponse:
        """
        Render the page.
        :return:
        """
        lang_crud = LanguageCRUD(self.engine)
        deck_crud = DecksCRUD(self.engine)

        template_params = {
            "language_id": self.language_id
        }
        if self.deck_id is not None:
            deck = await deck_crud.get_one(self.deck_id)
            template_params.update(
                language_id=deck.language_id,
                deck=deck
            )
        elif self.language_id is None:
            languages = await lang_crud.list_items()
            template_params.update(languages=languages)

        rsp_content = self.template_env\
            .get_template("deck-edit.html.jinja2")\
            .render(**template_params)

        return HTMLResponse(rsp_content)
