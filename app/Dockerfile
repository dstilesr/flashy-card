FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip3 install --upgrade --no-cache-dir pip \
    && pip3 install --no-cache-dir -r requirements.txt \
    && pip3 install --no-cache-dir "uvicorn==0.24.0.post1"

COPY templates ./templates
COPY src ./src

WORKDIR /app/src
EXPOSE 3000

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "3000"]
