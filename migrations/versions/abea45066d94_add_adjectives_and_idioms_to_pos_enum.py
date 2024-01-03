"""Add adjectives and idioms to POS enum

Revision ID: abea45066d94
Revises: 4a09bdedf6c3
Create Date: 2024-01-02 18:45:19.666402

"""
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision: str = 'abea45066d94'
down_revision: Union[str, None] = '4a09bdedf6c3'
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    op.execute("""
    alter type partofspeech add value 'ADJECTIVE';
    """)
    op.execute("""
    alter type partofspeech add value 'IDIOM';
    """)


def downgrade() -> None:
    # Can't delete values from enums easily in postgres
    pass
