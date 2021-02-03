FROM openjdk:8-jre-slim
EXPOSE 9092 2181
WORKDIR /app
COPY target/kafka-standalone-1.12.66.jar .
ENTRYPOINT ["java","-jar","kafka-standalone-1.12.66.jar"]
