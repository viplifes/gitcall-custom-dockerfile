FROM --platform=linux/arm64 biplifes/llm-llama:latest as model

FROM nvidia/cuda:12.6.1-base-ubuntu24.04

COPY --from=model /model /model

RUN apt update -y && apt install -y nodejs npm

WORKDIR /app
COPY src .

RUN npm install

RUN addgroup --gid 501 usercode && \
    adduser --disabled-password \
    --gecos "" \
    --shell /usr/sbin/nologin \
    --ingroup usercode \
    --uid 501 \
    usercode

RUN chown -R usercode:usercode /app && \
    chmod 0750 /app && \
    chown -R usercode:usercode /model && \
    chmod 0750 /model


USER usercode

ENTRYPOINT ["node", "/app/main.js"]