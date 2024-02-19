FROM rust:alpine3.19@sha256:ec93a9ad3065df593645171a3aa6c47b55578914d2c232860260dbd27bb0cbc0 as builder

RUN apk add --no-cache build-base

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.19.1@sha256:6457d53fb065d6f250e1504b9bc42d5b6c65941d57532c072d929dd0628977d0

RUN apk update && apk upgrade --no-cache

COPY --from=builder /app/target/release/course-sense-stub /usr/local/bin/course-sense-stub

ENTRYPOINT ["/usr/local/bin/course-sense-stub"]


