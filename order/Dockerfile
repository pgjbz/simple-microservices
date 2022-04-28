FROM docker.io/openjdk:17 as builder
COPY . /usr/src/order
WORKDIR /usr/src/order
RUN curl "https://dlcdn.apache.org/maven/maven-3/3.8.5/binaries/apache-maven-3.8.5-bin.tar.gz" -o ./maven.tar.gz
RUN tar xvf maven.tar.gz
RUN ./apache-maven-3.8.5/bin/mvn package -DskipTests

FROM docker.io/openjdk:17-jdk-alpine
WORKDIR /app
COPY --from=builder /usr/src/order/target/order.jar .
ENTRYPOINT ["java", "-XX:+UseZGC", "-jar", "/app/order.jar"]


