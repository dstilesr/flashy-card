from fastapi import APIRouter, Form
from typing import Annotated, Optional
from fastapi.responses import HTMLResponse, RedirectResponse

from ..db import DB_ENGINE
from ..controllers.deck_detail import DeckDetail
from ..controllers.list_decks import ListDecksLang
from ..controllers.add_edit_deck import EditCardDeck
from ..controllers.add_edit_deck_page import AddEditDeckPage
from ..controllers.add_remove_from_deck import AddRemoveFromDeck
from ..controllers.manage_card_deck_page import AddRemoveCardsDeckPage

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


@deck_router.post("/add")
async def add_card_deck(
        name: Annotated[str, Form()],
        context: Annotated[str, Form()],
        language_id: Annotated[int, Form()]) -> RedirectResponse:
    """
    Add new card deck.
    :param name:
    :param context:
    :param language_id:
    :return:
    """
    handler = EditCardDeck(
        DB_ENGINE,
        name,
        context,
        language_id,
    )
    rsp = await handler.process_request()
    return rsp


@deck_router.get("/{deck_id}/edit")
async def edit_card_deck_page(deck_id: int) -> HTMLResponse:
    """
    Render page to edit a card deck.
    :param deck_id:
    :return:
    """
    handler = AddEditDeckPage(DB_ENGINE, deck_id=deck_id)
    rsp = await handler.process_request()
    return rsp


@deck_router.post("/{deck_id}/edit")
async def edit_card_deck(
        deck_id: int,
        name: Annotated[str, Form()],
        context: Annotated[str, Form()],
        language_id: Annotated[int, Form()]) -> RedirectResponse:
    """
    Edit existing card deck.
    :param name:
    :param context:
    :param language_id:
    :param deck_id:
    :return:
    """
    handler = EditCardDeck(
        DB_ENGINE,
        name,
        context,
        language_id,
        deck_id
    )
    rsp = await handler.process_request()
    return rsp


@deck_router.get("/{deck_id}/add-cards")
async def add_cards_page(deck_id: int) -> HTMLResponse:
    """
    Add cards page for a given deck.
    :param deck_id:
    :return:
    """
    handler = AddRemoveCardsDeckPage(DB_ENGINE, deck_id)
    rsp = await handler.process_request()
    return rsp


@deck_router.post("/{deck_id}/add-card")
async def add_card(
        deck_id: int,
        card_id: Annotated[int, Form()]) -> RedirectResponse:
    """
    Add a card to the deck.
    :param deck_id:
    :param card_id:
    :return:
    """
    handler = AddRemoveFromDeck(DB_ENGINE, deck_id, card_id, False)
    rsp = await handler.process_request()
    return rsp


@deck_router.get("/{deck_id}/remove-cards")
async def remove_cards_page(deck_id: int) -> HTMLResponse:
    """
    Remove cards page for a given deck.
    :param deck_id:
    :return:
    """
    handler = AddRemoveCardsDeckPage(DB_ENGINE, deck_id, True)
    rsp = await handler.process_request()
    return rsp


@deck_router.post("/{deck_id}/remove-card")
async def remove_card(
        deck_id: int,
        card_id: Annotated[int, Form()]) -> RedirectResponse:
    """
    Remove a card from the deck.
    :param deck_id:
    :param card_id:
    :return:
    """
    handler = AddRemoveFromDeck(DB_ENGINE, deck_id, card_id, True)
    rsp = await handler.process_request()
    return rsp


@deck_router.get("/{deck_id}/detail")
async def deck_detail(deck_id: int) -> HTMLResponse:
    """
    Render deck details page.
    :param deck_id:
    :return:
    """
    handler = DeckDetail(DB_ENGINE, deck_id)
    rsp = await handler.process_request()
    return rsp
