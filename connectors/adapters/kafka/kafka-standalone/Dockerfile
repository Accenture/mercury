FROM mcr.microsoft.com/openjdk/jdk:11-ubuntu
EXPOSE 9092
WORKDIR /app
COPY target/kafka-standalone-3.0.9.jar .
ENTRYPOINT ["java","-jar","kafka-standalone-3.0.9.jar"]
