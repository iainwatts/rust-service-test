FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/rocket-tests
COPY . .

RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install --path .

CMD ["rocket-tests"]

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl postgresql-client
COPY --from=builder /usr/local/cargo/bin/rocket-tests /usr/local/bin/rocket-tests
EXPOSE 8000
CMD ["rocket-tests"]
