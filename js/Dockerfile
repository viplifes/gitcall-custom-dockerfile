FROM node:22-alpine as builder

WORKDIR /app
COPY src src

ENTRYPOINT ["node", "/app/src/main.js"]