FROM openjdk:11-jre-slim
EXPOSE 9092
WORKDIR /app
COPY target/kafka-standalone-2.3.6.jar .
ENTRYPOINT ["java","-jar","kafka-standalone-2.3.6.jar"]
