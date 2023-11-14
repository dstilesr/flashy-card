from fastapi import FastAPI

app = FastAPI(
    title="Flashy Card App",
    description="Flash cards for language learning."
)


@app.get("/health")
async def health():
    return {"status": "ok"}
