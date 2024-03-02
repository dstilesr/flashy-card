import sqlalchemy as sa
from datetime import datetime
from typing import List, Optional
from sqlalchemy.orm import Mapped, mapped_column, relationship

from .base import Base
from ..enums import StudyType
from .main_tables import FlashCard, CardDeck


class CardStudy(Base):
    """
    Represents a 'study' instance for a flash card.
    """
    __tablename__ = "card_studies"

    id: Mapped[int] = mapped_column(
        sa.BigInteger(),
        autoincrement=True,
        primary_key=True
    )
    succeeded: Mapped[bool] = mapped_column()

    card_id: Mapped[int] = mapped_column(sa.ForeignKey("flash_cards.id"))
    study_run_id: Mapped[int] = mapped_column(sa.ForeignKey("study_runs.id"))

    # Dates
    created_at: Mapped[datetime] = mapped_column(server_default=sa.func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=sa.func.now())

    # Relationships
    card: Mapped[FlashCard] = relationship(back_populates="studies")
    study_run: Mapped["StudyRun"] =\
        relationship(back_populates="card_studies")


class StudyRun(Base):
    """
    A study run of a card deck.
    """
    __tablename__ = "study_runs"

    id: Mapped[int] = mapped_column(
        sa.BigInteger(),
        autoincrement=True,
        primary_key=True
    )
    completed: Mapped[bool] = mapped_column(default=False)
    random_seed: Mapped[int] = mapped_column(sa.BigInteger)
    card_deck_id: Mapped[int] = mapped_column(sa.ForeignKey("card_decks.id"))
    run_type: Mapped[StudyType] = mapped_column(sa.types.Enum(StudyType))

    # Dates
    started_at: Mapped[datetime] = mapped_column(server_default=sa.func.now())
    ended_at: Mapped[Optional[datetime]] = mapped_column(nullable=True)

    # Relationships
    deck: Mapped[CardDeck] = relationship(back_populates="study_runs")
    card_studies: Mapped[List[CardStudy]] = relationship(back_populates="study_run")
