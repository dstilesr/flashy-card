from fastapi import APIRouter
from fastapi.responses import HTMLResponse, JSONResponse

from ..template_util import TEMPLATE_ENV

base_router = APIRouter()


@base_router.get("/")
async def main_page() -> HTMLResponse:
    """
    Main page of the app.
    :return:
    """
    page_text = TEMPLATE_ENV.get_template("main.html.jinja2").render()
    return HTMLResponse(
        page_text,
        status_code=200
    )


@base_router.get("/health")
async def health() -> JSONResponse:
    """
    Simple health-check endpoint.
    :return:
    """
    return JSONResponse(
        {"status": "ok"},
        status_code=200
    )
