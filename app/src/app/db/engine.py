import os
from typing import Optional
from sqlalchemy.ext.asyncio import create_async_engine, AsyncEngine

from .settings import DBSettings


def get_engine(settings: Optional[DBSettings] = None) -> AsyncEngine:
    """
    Get the SQL engine.
    :param settings:
    :return:
    """
    if settings is None:
        settings = DBSettings()

    return create_async_engine(
        "postgresql+asyncpg://%s:%s@%s/%s"
        % (
            settings.user,
            settings.password,
            settings.host,
            settings.db
        ),
        pool_size=settings.pool_size
   )

DB_ENGINE = get_engine()
