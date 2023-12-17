"""Initial table creation

Revision ID: 0fa7bfc09aae
Revises: 
Create Date: 2023-11-15 09:18:28.552632

"""
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision: str = '0fa7bfc09aae'
down_revision: Union[str, None] = None
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.create_table(
        'languages',
        sa.Column('id', sa.BigInteger(), autoincrement=True, nullable=False),
        sa.Column('name', sa.String(length=128), nullable=False),
        sa.Column('slug', sa.String(length=64), nullable=False),
        sa.Column('notes', sa.Text(), nullable=True),
        sa.PrimaryKeyConstraint('id'),
        sa.UniqueConstraint('slug')
    )
    op.create_index('language_slug_idx', 'languages', ['slug'], unique=True)
    op.create_table(
        'card_decks',
        sa.Column('id', sa.BigInteger(), autoincrement=True, nullable=False),
        sa.Column('name', sa.String(length=256), nullable=False),
        sa.Column('context', sa.String(length=256), nullable=False),
        sa.Column('language_id', sa.BigInteger(), nullable=False),
        sa.Column('created_at', sa.DateTime(), server_default=sa.text('now()'), nullable=False),
        sa.Column('updated_at', sa.DateTime(), server_default=sa.text('now()'), nullable=False),
        sa.ForeignKeyConstraint(['language_id'], ['languages.id'], ),
        sa.PrimaryKeyConstraint('id')
    )
    op.create_table(
        'flash_cards',
        sa.Column('id', sa.BigInteger(), autoincrement=True, nullable=False),
        sa.Column('target_word', sa.String(length=64), nullable=False),
        sa.Column('translation', sa.String(length=256), nullable=False),
        sa.Column('sentence', sa.String(length=256), nullable=True),
        sa.Column('target_language_id', sa.BigInteger(), nullable=False),
        sa.Column(
            'part_of_speech',
            sa.Enum('NOUN', 'VERB', 'CONJUNCTION', 'PREPOSITION', 'ADVERB', name='partofspeech'),
            nullable=False
        ),
        sa.Column('created_at', sa.DateTime(), server_default=sa.text('now()'), nullable=False),
        sa.Column('updated_at', sa.DateTime(), server_default=sa.text('now()'), nullable=False),
        sa.Column('deleted_at', sa.DateTime(), nullable=True),
        sa.ForeignKeyConstraint(['target_language_id'], ['languages.id'], ),
        sa.PrimaryKeyConstraint('id')
    )
    op.create_table(
        'card_deck_mapping',
        sa.Column('card_id', sa.BigInteger(), nullable=True),
        sa.Column('deck_id', sa.BigInteger(), nullable=True),
        sa.ForeignKeyConstraint(['card_id'], ['flash_cards.id'], ),
        sa.ForeignKeyConstraint(['deck_id'], ['card_decks.id'], )
    )
    # ### end Alembic commands ###


def downgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.drop_table('card_deck_mapping')
    op.drop_table('flash_cards')
    op.drop_table('card_decks')
    op.drop_index('language_slug_idx', table_name='languages')
    op.drop_table('languages')
    # ### end Alembic commands ###