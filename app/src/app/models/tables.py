import sqlalchemy as sa
from datetime import datetime
from typing import Optional, List
from sqlalchemy.orm import Mapped, mapped_column, relationship

from .base import Base
from ..enums import PartOfSpeech


class Language(Base):
    """
    A target language...
    """
    __tablename__ = "languages"

    id: Mapped[int] = mapped_column(
        sa.BigInteger(),
        autoincrement=True,
        primary_key=True
    )
    name: Mapped[str] = mapped_column(sa.String(128), nullable=False)
    notes: Mapped[Optional[str]] = mapped_column(
        sa.Text(),
        nullable=True
    )


class FlashCard(Base):
    """
    A flash card containing a word in the target language, its definition in
    a source language, and a phrase with the word in context.
    """
    __tablename__ = "flash_cards"

    id: Mapped[int] = mapped_column(
        sa.BigInteger(),
        autoincrement=True,
        primary_key=True
    )
    target_word: Mapped[str] = mapped_column(sa.String(64))
    translation: Mapped[str] = mapped_column(sa.String(256))
    sentence: Mapped[Optional[str]] = mapped_column(
        sa.String(256),
        nullable=True
    )
    target_language_id: Mapped[int] = mapped_column(
        sa.ForeignKey("languages.id")
    )
    part_of_speech: Mapped[PartOfSpeech] = mapped_column(
        sa.types.Enum(PartOfSpeech)
    )

    created_at: Mapped[datetime] = mapped_column(server_default=sa.func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=sa.func.now())
    deleted_at: Mapped[Optional[datetime]]

    target_language: Mapped["Language"] = relationship()
