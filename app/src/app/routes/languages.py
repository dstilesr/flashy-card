from fastapi import APIRouter
from fastapi.responses import HTMLResponse

from ..db import DB_ENGINE
from ..controllers.list_languages import ListLanguages

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
