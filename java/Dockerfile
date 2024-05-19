
# FROM docker-hub.middleware.biz/public/gitcall/java-runner:0.1.5 as runner

FROM amazoncorretto:21-alpine

WORKDIR /app
COPY gradle gradle
COPY settings.gradle settings.gradle
COPY gradlew gradlew
COPY runner runner
#  RESULT=$(javac -cp "/runner/*" -d build/ ${USERCODE_CLASS_FILE} 2>&1)
RUN ./gradlew build

# RUN ./gradlew runner:make_distribution && \
# tar -xvf runner/build/distributions/runner.tar -C /
# RUN apk update && apk add tar && \
#   ./gradlew runner:make_distribution && \
#   tar -xvf runner/build/distributions/runner.tar -C /





# RUN javac -d build/ "runner/src/main/java/com/corezoid/gitcall/runner/Gitcall.java"
# RUN javac -d mods/ src/gitcall/*.java src/module-info.java
# RUN jlink --module-path $JAVA_HOME/jmods:mods --add-modules gitcall --output gitcallimg

# ENTRYPOINT ["./gradlew", "run"]
# ENTRYPOINT ["sh"]
ENTRYPOINT ["java", "-jar", "runner/build/libs/runner.jar"]
# ENTRYPOINT ["java", "-cp", "/runner/*:/usercode/usercode.jar", "com.corezoid.gitcall.runner.Gitcall"]

# ENTRYPOINT ["/usercode-proxy", "--", "java", "-cp", "/runner/*:/usercode/usercode.jar", "com.corezoid.runner.Runner"]