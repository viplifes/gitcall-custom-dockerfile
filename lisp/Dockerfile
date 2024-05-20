FROM fukamachi/sbcl:2.4.4 as builder

RUN ros install clack jonathan
WORKDIR /app
COPY src src

RUN ros build /app/src/main.ros

ENTRYPOINT ["/app/src/main"]