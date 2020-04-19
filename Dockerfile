# Build
FROM rust:1.42-alpine as build

WORKDIR /app
COPY ./ ./
RUN cargo build --release


# Final image
FROM alpine:latest

ENV INPUT_DIR /input
ENV LIBRARY_DIR /library

COPY --from=build /app/target/release/media-organizer .
CMD ["./media-organizer"]
