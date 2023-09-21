FROM openjdk:11-jre-slim
EXPOSE 9092
WORKDIR /app
COPY target/kafka-standalone-3.0.5.jar .
ENTRYPOINT ["java","-jar","kafka-standalone-3.0.5.jar"]
