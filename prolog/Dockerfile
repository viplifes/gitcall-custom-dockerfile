FROM swipl:9.2.4 as builder

WORKDIR /app
COPY src src

RUN addgroup --gid 501 usercode && \
    adduser --disabled-password \
    --gecos "" \
    --shell /usr/sbin/nologin \
    --ingroup usercode \
    --uid 501 \
    usercode
USER usercode

CMD ["swipl", "src/main.pl"]