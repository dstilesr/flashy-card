from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class DBSettings(BaseSettings):
    """
    Database settings.
    """
    host: str = Field(description="Database host.")
    user: str = Field(description="Database user.")
    password: str = Field(description="Database password.")
    db: str = Field(description="Database name.")
    port: int = Field(description="Database port.", default=5432)
    pool_size: int = Field(
        description="Database connection pool size.",
        default=5
    )

    model_config = SettingsConfigDict(
        env_prefix="POSTGRES_",
    )
