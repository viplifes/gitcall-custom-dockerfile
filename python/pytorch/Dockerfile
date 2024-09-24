FROM pytorch/pytorch:2.4.1-cuda11.8-cudnn9-runtime

WORKDIR /app
COPY requirements.txt requirements.txt
RUN pip install -r requirements.txt
COPY src src

RUN addgroup --gid 501 usercode && \
    adduser --disabled-password \
    --gecos "" \
    --shell /usr/sbin/nologin \
    --ingroup usercode \
    --uid 501 \
    usercode
USER usercode

ENTRYPOINT fastapi run src/main.py --host=0.0.0.0 --port="$GITCALL_PORT"