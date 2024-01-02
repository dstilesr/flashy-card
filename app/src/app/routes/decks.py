from fastapi import APIRouter, Form
from typing import Annotated, Optional
from fastapi.responses import HTMLResponse, RedirectResponse

from ..db import DB_ENGINE
from ..controllers.list_decks import ListDecksLang
from ..controllers.add_edit_deck_page import AddEditDeckPage

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


@deck_router.get("/add")
async def add_card_deck(language_id: Optional[int] = None) -> HTMLResponse:
    """
    Page to add a new card deck.
    :param language_id:
    :return:
    """
    handler = AddEditDeckPage(DB_ENGINE, language_id=language_id)
    rsp = await handler.process_request()
    return rsp


@deck_router.get("/{deck_id}/edit")
async def edit_card_deck(deck_id: int) -> HTMLResponse:
    """
    Render page to edit a card deck.
    :param deck_id:
    :return:
    """
    handler = AddEditDeckPage(DB_ENGINE, deck_id=deck_id)
    rsp = await handler.process_request()
    return rsp
