import sqlalchemy as sa
import sqlalchemy.ext.asyncio as sa_async
from typing import Dict, Any, AsyncIterable, Optional

from sqlalchemy.ext.asyncio import AsyncSession

from .base import BaseCrud
from ..utils import to_slug
from ..models import Language
from .. import exceptions as err


class LanguageCRUD(BaseCrud):
    """
    Handler for language model crud.
    """

    async def get_one(
            self,
            session: AsyncSession,
            slug: Optional[str] = None,
            language_id: Optional[int] = None) -> Language:
        """
        Get a language from the DB.
        :param session:
        :param slug:
        :param language_id:
        :return:
        """
        if slug is None and language_id is None:
            raise err.InvalidParameters(
                "Must provide either language slug or ID."
            )

        stmt = sa.select(Language)
        if language_id is not None:
            stmt = stmt.where(Language.id == language_id)
        else:
            stmt = stmt.where(Language.slug == slug)
        res = await session.scalars(stmt)
        lng = res.one_or_none()
        if lng is None:
            raise err.ResourceNotFound("Language not found!")
        return lng

    async def create(
            self,
            session: sa_async.AsyncSession,
            parameters: Dict[str, Any]):
        """
        Insert language record.
        :param session:
        :param parameters: Dictionary with name, notes.
        :return:
        """
        # Check if it already exists
        slug = to_slug(parameters["name"])
        results = await session.scalars(
            sa.select(Language).where(Language.slug == slug)
        )
        existing = results.one_or_none()
        if existing is not None:
            raise err.ResourceExists(
                "Language '%s' already in the database." % slug
            )

        lng = Language(
            name=parameters["name"],
            slug=slug,
            notes=parameters.get("notes")
        )
        session.add(lng)
        await session.commit()

    async def list_items(
            self,
            session: sa_async.AsyncSession,
            **filters) -> AsyncIterable[Language]:
        """
        List available languages.
        :param session:
        :param filters: Not used.
        :return:
        """
        stream = await session.stream_scalars(sa.select(Language))
        return stream
