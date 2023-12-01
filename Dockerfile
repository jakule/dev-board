# Build the nodejs frontend

FROM node:18-alpine as ui-build

WORKDIR /app

COPY package.json package-lock.json ./
COPY web/package.json web/package-lock.json ./web/

RUN npm install --dev

COPY web web/

RUN npm run build

# Build the rust backend

FROM rust:1.74-bookworm as backend

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN \
    mkdir /app/src && \
    echo 'fn main() {}' > /app/src/main.rs && \
    cargo build --release && \
    rm -Rvf /app/src

COPY . .

COPY --from=ui-build /app/web/ ./web/web/build

ENV SQLX_OFFLINE=true
RUN touch /app/src/main.rs && cargo build --release

# Build the runtime image

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=backend /app/target/release/dev-board .

EXPOSE 5800

CMD [ "./dev-board", "--config", "/app/config/config-docker.toml" ]