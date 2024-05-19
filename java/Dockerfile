
FROM amazoncorretto:21-alpine

WORKDIR /app
COPY gradle gradle
COPY settings.gradle settings.gradle
COPY gradlew gradlew
COPY runner runner

RUN ./gradlew build

ENTRYPOINT ["java", "-jar", "runner/build/libs/runner.jar"]