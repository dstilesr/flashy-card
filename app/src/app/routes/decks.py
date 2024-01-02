from fastapi import APIRouter, Form
from typing import Annotated, Optional
from fastapi.responses import HTMLResponse, RedirectResponse

from ..db import DB_ENGINE
from ..controllers.list_decks import ListDecksLang

deck_router = APIRouter()


@deck_router.get("/{language_slug}/list")
async def list_decks(language_slug: str) -> HTMLResponse:
    """
    List flash card decks page for the given language.
    :param language_slug:
    :return:
    """
    handler = ListDecksLang(DB_ENGINE, language_slug)
    rsp = await handler.process_request()
    return rsp
