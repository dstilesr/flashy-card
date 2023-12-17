from fastapi import APIRouter, Form
from typing import Annotated, Optional
from fastapi.responses import HTMLResponse, RedirectResponse

from ..db import DB_ENGINE
from ..enums import PartOfSpeech
from ..controllers.add_edit_card import EditCard
from ..controllers.list_cards import ListCardsLang
from ..controllers.edit_card_page import EditCardPage

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


@card_router.get("/add")
async def add_card_page(language_id: Optional[int] = None) -> HTMLResponse:
    """
    Page for adding new card form.
    :param language_id:
    :return:
    """
    handler = EditCardPage(DB_ENGINE, language_id=language_id)
    rsp = await handler.process_request()
    return rsp


@card_router.post("/add")
async def add_card(
        word: Annotated[str, Form()],
        translation: Annotated[str, Form()],
        language_id: Annotated[int, Form()],
        part_of_speech: Annotated[PartOfSpeech, Form()],
        sentence: Annotated[Optional[str], Form()] = None) -> RedirectResponse:
    """
    Add a new flash card.
    :param word:
    :param translation:
    :param language_id:
    :param part_of_speech:
    :param sentence:
    :return:
    """
    handler = EditCard(
        DB_ENGINE,
        word=word,
        translation=translation,
        language_id=language_id,
        part_of_speech=part_of_speech,
        sentence=sentence if sentence != "" else None
    )
    rsp = await handler.process_request()
    return rsp


@card_router.post("/{card_id}/edit")
async def edit_card(
        card_id: int,
        word: Annotated[str, Form()],
        translation: Annotated[str, Form()],
        language_id: Annotated[int, Form()],
        part_of_speech: Annotated[PartOfSpeech, Form()],
        sentence: Annotated[Optional[str], Form()] = None) -> RedirectResponse:
    """
    Add a new flash card.
    :param word:
    :param translation:
    :param language_id:
    :param part_of_speech:
    :param card_id:
    :param sentence:
    :return:
    """
    handler = EditCard(
        DB_ENGINE,
        word=word,
        translation=translation,
        language_id=language_id,
        part_of_speech=part_of_speech,
        sentence=sentence if sentence != "" else None,
        card_id=card_id
    )
    rsp = await handler.process_request()
    return rsp
