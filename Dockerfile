ARG RUST_VERSION=1.84.0
ARG APP_NAME=actix-testing

FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
COPY . /app
WORKDIR /app
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM ubuntu:25.04 AS final
RUN apt update && apt install -y sqlite3 libsqlite3-0
COPY --from=build /bin/server /bin/
WORKDIR /app
EXPOSE 8080
ENV environment="docker"
CMD ["/bin/server"]