FROM python:3.12-alpine as builder

WORKDIR /app
COPY requirements.txt requirements.txt
RUN pip install -r requirements.txt
COPY src src

ENTRYPOINT ["python", "src/main.py"]