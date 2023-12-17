from typing import Annotated
from fastapi import APIRouter, Form
from fastapi.responses import HTMLResponse, RedirectResponse

from ..db import DB_ENGINE
from ..controllers.list_cards import ListCardsLang

card_router = APIRouter()


@card_router.get("/{language_slug}/list")
async def list_cards_language(language_slug: str) -> HTMLResponse:
    """
    List the cards for the given language.
    :param language_slug:
    :return:
    """
    controller = ListCardsLang(DB_ENGINE, language_slug=language_slug)
    rsp = await controller.process_request()
    return rsp
