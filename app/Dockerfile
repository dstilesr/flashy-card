FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt \
    && pip install --no-cache-dir "uvicorn==0.24.0.post1"

COPY templates ./templates
COPY src ./src

WORKDIR /app/src

CMD ["uvicorn", "main:app", "--host", "0.0.0.0"]
