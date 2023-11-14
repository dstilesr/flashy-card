import os
from sqlalchemy.ext.asyncio import create_async_engine

DB_ENGINE = create_async_engine(
    "postgresql+asyncpg://%s:%s@%s/%s"
    % (
        os.getenv("POSTGRES_USER"),
        os.getenv("POSTGRES_PASSWORD"),
        os.getenv("DB_HOST"),
        os.getenv("POSTGRES_DB")
    ),
    pool_size=5
)
