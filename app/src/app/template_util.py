import jinja2
from pathlib import Path

TEMPLATES_PATH: Path = Path(__file__).parents[2] / "templates"

TEMPLATE_ENV = jinja2.Environment(
    loader=jinja2.FileSystemLoader(TEMPLATES_PATH)
)
