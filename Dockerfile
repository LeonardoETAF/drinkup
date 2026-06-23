# syntax=docker/dockerfile:1

# ─────────────────────────── Estágio de build ───────────────────────────
# Compila o WASM (hidratação) + o binário do servidor com cargo-leptos.
FROM rust:1.96-bookworm AS builder

# Alvo wasm e cargo-leptos (o wasm-opt é baixado pelo próprio cargo-leptos).
RUN rustup target add wasm32-unknown-unknown \
 && cargo install cargo-leptos --version 0.3.6 --locked

WORKDIR /app

# Cache de dependências: com stubs (o manifesto exige lib+bin), baixa o registry
# antes de copiar o código-fonte real.
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
 && echo '' > src/lib.rs \
 && echo 'fn main() {}' > src/main.rs \
 && cargo fetch \
 && rm -rf src

# Código, estilos, assets, migrations e cache de queries (.sqlx).
COPY . .

# Domínio público do site (canonical/OG/sitemap/links de e-mail), bakeado no
# build. No EasyPanel: Build Arguments → DRINKUP_SITE_URL=https://teste.drinkup.com.br
# Vazio/ausente cai no padrão de produção (ver src/components/seo.rs).
ARG DRINKUP_SITE_URL=""
ENV DRINKUP_SITE_URL=${DRINKUP_SITE_URL}

# Build de produção SEM banco: as queries são checadas pelo cache `.sqlx`.
ENV SQLX_OFFLINE=true
RUN cargo leptos build --release

# ─────────────────────────── Imagem final ───────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates wget \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --create-home --uid 10001 drinkup

WORKDIR /app

# Binário do servidor + assets gerados (JS/WASM/CSS + públicos).
COPY --from=builder /app/target/release/drinkup /app/drinkup
COPY --from=builder /app/target/site /app/site

# Diretório de uploads (montar volume aqui para persistir entre deploys).
RUN mkdir -p /app/uploads && chown -R drinkup:drinkup /app
USER drinkup

# `LEPTOS_*` apontam para os assets dentro da imagem; `0.0.0.0` para aceitar
# conexões do proxy reverso. Segredos/DB vêm do ambiente (compose).
ENV LEPTOS_OUTPUT_NAME=drinkup \
    LEPTOS_SITE_ROOT=site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    LEPTOS_ENV=PROD \
    RUST_LOG=info

EXPOSE 3000

# Healthcheck simples na home.
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
  CMD wget -qO- http://127.0.0.1:3000/ >/dev/null 2>&1 || exit 1

CMD ["/app/drinkup"]
