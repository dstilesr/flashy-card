"""Tables for Study Runs

Revision ID: 0f8384d82b5b
Revises: abea45066d94
Create Date: 2024-01-16 19:11:16.670769

"""
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision: str = '0f8384d82b5b'
down_revision: Union[str, None] = 'abea45066d94'
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.create_table(
        'study_runs',
        sa.Column('id', sa.BigInteger(), autoincrement=True, nullable=False),
        sa.Column('completed', sa.Boolean(), nullable=False),
        sa.Column('random_seed', sa.BigInteger(), nullable=False),
        sa.Column('card_deck_id', sa.BigInteger(), nullable=False),
        sa.Column('started_at', sa.DateTime(), server_default=sa.text('now()'), nullable=False),
        sa.Column('ended_at', sa.DateTime(), nullable=True),
        sa.ForeignKeyConstraint(['card_deck_id'], ['card_decks.id'], name="study_run_deck_fk"),
        sa.PrimaryKeyConstraint('id')
    )
    op.create_table(
        'card_studies',
        sa.Column('id', sa.BigInteger(), autoincrement=True, nullable=False),
        sa.Column('succeeded', sa.Boolean(), nullable=False),
        sa.Column('card_id', sa.BigInteger(), nullable=False),
        sa.Column('study_run_id', sa.BigInteger(), nullable=False),
        sa.Column('created_at', sa.DateTime(), server_default=sa.text('now()'), nullable=False),
        sa.Column('updated_at', sa.DateTime(), server_default=sa.text('now()'), nullable=False),
        sa.ForeignKeyConstraint(['card_id'], ['flash_cards.id'], "card_study_card_fk"),
        sa.ForeignKeyConstraint(['study_run_id'], ['study_runs.id'], "card_study_run_fk"),
        sa.PrimaryKeyConstraint('id')
    )
    # ### end Alembic commands ###


def downgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.drop_table('card_studies')
    op.drop_table('study_runs')
    # ### end Alembic commands ###