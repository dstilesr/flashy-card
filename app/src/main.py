from fastapi import FastAPI
from app.controllers import base_router

app = FastAPI(
    title="Flashy Card App",
    description="Flash cards for language learning."
)

app.include_router(base_router)
