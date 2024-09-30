FROM biplifes/llm-llama:latest

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