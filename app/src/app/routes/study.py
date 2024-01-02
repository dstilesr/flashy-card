from typing import Optional
from fastapi import APIRouter
from fastapi.responses import HTMLResponse

from ..db import DB_ENGINE
from ..controllers.study_cards import StudyCardPage

study_router = APIRouter()


@study_router.get("/{deck_id}/from-target")
async def study_from_tgt_lang(
        deck_id: int,
        seed: Optional[int] = None,
        idx: int = 1,
        show_example: bool = False,
        show_answer: bool = False) -> HTMLResponse:
    """
    Study a card from the target to the source language.
    :param deck_id:
    :param seed:
    :param idx:
    :param show_example:
    :param show_answer:
    :return:
    """
    handler = StudyCardPage(
        DB_ENGINE,
        deck_id,
        idx,
        show_example,
        show_answer,
        to_target=False,
        rnd_seed=seed
    )
    rsp = await handler.process_request()
    return rsp


@study_router.get("/{deck_id}/to-target")
async def study_to_tgt_lang(
        deck_id: int,
        seed: Optional[int] = None,
        idx: int = 1,
        show_example: bool = False,
        show_answer: bool = False) -> HTMLResponse:
    """
    Study a card from the source to the target language.
    :param deck_id:
    :param seed:
    :param idx:
    :param show_example:
    :param show_answer:
    :return:
    """
    handler = StudyCardPage(
        DB_ENGINE,
        deck_id,
        idx,
        show_example,
        show_answer,
        to_target=True,
        rnd_seed=seed
    )
    rsp = await handler.process_request()
    return rsp
