import sqlalchemy as sa
from datetime import datetime
from typing import Dict, Any, Sequence, Optional

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
            slug: Optional[str] = None,
            language_id: Optional[int] = None) -> Language:
        """
        Get a language from the DB.
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

        async with self.get_session() as session:
            res = await session.scalars(stmt)
            lng = res.one_or_none()
            if lng is None:
                raise err.ResourceNotFound("Language not found!")
        return lng

    async def create(
            self,
            parameters: Dict[str, Any],
            refresh: bool = False) -> Language:
        """
        Insert language record.
        :param parameters: Dictionary with name, notes.
        :param refresh:
        :return:
        """
        # Check if it already exists
        slug = to_slug(parameters["name"])
        async with self.get_session() as session:
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
            if refresh:
                await session.refresh(lng)
            return lng

    async def list_items(
            self,
            **filters) -> Sequence[Language]:
        """
        List available languages.
        :param filters: Not used.
        :return:
        """
        async with self.get_session() as session:
            results = await session.scalars(
                sa.select(Language).where(Language.deleted_at.is_(None))
            )
        return results

    async def delete(
            self,
            slug: str):
        """
        Soft-delete a language from the DB.
        :return:
        """
        stmt = sa.update(Language)\
            .where((Language.slug == slug) & (Language.deleted_at.is_(None)))\
            .values(deleted_at=datetime.utcnow(), slug=f"{slug}-deleted-lang")

        async with self.get_session() as session:
            res = await session.execute(stmt)
            if res.rowcount == 0:
                raise err.ResourceNotFound("Found no 'Language' to delete!")

            await session.commit()
