from typing import Annotated
from fastapi import APIRouter, Form
from fastapi.responses import HTMLResponse, RedirectResponse

from ..db import DB_ENGINE
from ..controllers.list_languages import ListLanguages
from ..controllers.add_language import AddLanguagePage, AddLanguagePost

lang_router = APIRouter()


@lang_router.get("/list")
async def list_languages() -> HTMLResponse:
    """
    Languages list page.
    :return:
    """
    handler = ListLanguages(DB_ENGINE)
    rsp = await handler.process_request()
    return rsp


@lang_router.get("/add")
async def add_language_page() -> HTMLResponse:
    """
    Page to add a new language.
    :return:
    """
    handler = AddLanguagePage(DB_ENGINE)
    rsp = await handler.process_request()
    return rsp


@lang_router.post("/add")
async def add_language(
        lang_name: Annotated[str, Form()],
        lang_notes: Annotated[str, Form()]
        ) -> RedirectResponse:
    """
    Add a new language to the DB.
    :param lang_notes:
    :param lang_name:
    :return:
    """
    handler = AddLanguagePost(DB_ENGINE)
    rsp = await handler.process_request(lang_name, lang_notes)
    return rsp
