# DRINKUP — Prompts de Desenvolvimento

Roteiro sequencial. Cada fase é um prompt pronto para ser executado (na ordem).
**Todas as fases obedecem ao `CLAUDE.md`** — principalmente o princípio do **frontend burro** e as regras de segurança/desempenho. Não pular a fundação.

Antes de começar: coloque os materiais de design em `assets/` (`logos/`, `icons/`, `backgrounds/`, `design/`). As fases de UI dependem deles.

### Escopo real (descoberto nos mockups em `assets/design/`)
O projeto é maior que um catálogo puro. Seções a contemplar nas Fases 2, 4 e 7:

- **Site (público):** home, products, product (detalhe), **about**, **partners**, contact.
- **Painel (admin):** dashboard, products, **events**, leads, **partners**, settings.
- **Marca:** logos branca/preta + ícone, slogan "CHEERS TO LIFE", backgrounds (azul/verde/doodles).

Ou seja, além do catálogo há **eventos** e **parceiros**. O modelo de dados (Fase 2) e as
telas (Fases 4 e 7) devem cobrir essas entidades. O escopo continua **sem pagamento online**
(vitrine + orçamento/contato/leads).

---

## Status (atualizado em 2026-06-18, após Fase 6)

| Fase | Estado | Notas |
|------|--------|-------|
| 0 — Scaffold | ✅ feito | Leptos 0.8 SSR + Axum; `#![forbid/deny(unsafe_code)]` (exceção isolada do wasm-bindgen em `hydration.rs`). |
| 1 — Tokens + base visual | ✅ feito | Tokens reais extraídos dos mockups; Montserrat auto-hospedada (**fonte a confirmar no guia**); header (com hambúrguer) e footer. |
| 2 — Banco + migrations | ✅ feito | 8 migrations (categorias, produtos+imagens, usuarios, leads+itens, eventos, parceiros, audit_log); pool + `migrate!`; seeds. **Sessões adiadas p/ Fase 6.** |
| 3 — Domínio + server functions | ✅ feito | DTOs em `domain/`; repositórios em `server/` com `query!`/`query_as!`; camada `api/` `#[server]`; cache `.sqlx` commitável. |
| 4 — Vitrine (catálogo) | ✅ feito | Home (hero+destaques+CTA), `/produtos` (filtros/busca/paginação via query params, SSR), `/produtos/:slug` (galeria+atributos). Placeholders p/ quem-somos/parceiros. |
| 5 — Orçamento/contato | ✅ feito | `/contato` com validação cliente+servidor, honeypot + throttle, prefill por produto; teste de integração passando. |
| 6 — Auth + RBAC | ✅ feito | Argon2id; sessão `tower-sessions` (store Postgres, cookie HttpOnly/SameSite/Secure-em-prod); rate limit no login; guarda `exigir_papel`; middleware 303 em `/admin/*`; auditoria de login/logout. CLI `src/bin/admin.rs` cria o admin. |
| 7 — Painel admin | 🔄 em andamento | **Parte 1**: casca (sidebar+topo) + dashboard + leads (listar/buscar/filtrar/mudar status). **Parte 2a**: CRUD de produtos (listar/buscar/criar/editar/excluir, slug automático). RBAC (`exigir_papel`) em toda server fn + middleware 303. Dados do painel carregam **client-side** (Effect+spawn_local) — acessar sessão/banco no SSR com streaming trava (tower-sessions). **Pendente**: 2b upload de imagens, eventos, parceiros, usuários/config. |
| 8 — Endurecimento de segurança | ⬜ pendente | CSP/HSTS, CSRF, rate limit por IP, auditoria do bundle. |
| 9 — SEO + desempenho | ⬜ pendente | meta tags, sitemap, otimização de imagens, cache. |
| 10 — Deploy (VPS/Docker) | ⬜ pendente | Dockerfile multi-stage, compose de produção. |

**Pendências de decisão registradas:** fonte oficial da marca (usando Montserrat); provedor de e-mail/notificação de leads (hoje só registra no banco); imagens reais de produto (placeholder até o painel).

**Saúde do código (último check):** `cargo leptos build`, `clippy` (ssr+wasm), `fmt` e **3 testes** (auth × 2, orçamento × 1) limpos. Repositório git inicializado (branch `main`); **Fase 6 ainda não commitada**.

**Criar admin (dev/prod):** `cargo run --bin admin --no-default-features --features ssr -- <email> <nome> <senha> [papel]`.

---

## Fase 0 — Scaffold do projeto

> Crie o esqueleto do projeto Leptos 0.8 SSR full-stack com Axum, conforme o `CLAUDE.md`.
> - `Cargo.toml` com crate único e features `ssr`/`hydrate`, dependências: leptos, leptos_axum, axum, tokio, tower, tower-sessions, sqlx (postgres, runtime-tokio, macros), argon2, serde, thiserror, tracing.
> - `rust-toolchain.toml`, `.gitignore`, `.env.example` (com as variáveis da seção 9 do CLAUDE.md).
> - Estrutura de pastas da seção 6 do CLAUDE.md (criar diretórios e arquivos `mod` vazios).
> - `main.rs` (servidor Axum + Leptos), `lib.rs` (hydrate), `app.rs` (shell + Router com rota raiz placeholder).
> - `style/main.css` importando `tokens.css`/`base.css` vazios.
> - `docker-compose.yml` com serviço `db` (Postgres) para desenvolvimento.
> - `#![forbid(unsafe_code)]` no crate e estrutura modular (módulos por responsabilidade) desde o início, conforme seção 7 do CLAUDE.md.
> Garanta que `cargo leptos watch` sobe e exibe uma página "Hello DRINKUP". Rode `cargo fmt` e `cargo clippy` (sem warnings).

---

## Fase 1 — Design tokens e base visual

> A partir dos materiais em `assets/design/`, `logos/`, `icons/`, `backgrounds/`:
> - Extraia a paleta de cores, tipografia, espaçamentos, raios e sombras para `style/_tokens.scss` como variáveis CSS (design tokens).
> - Monte `style/_base.scss` (reset moderno, tipografia base, foco visível, comportamento responsivo mobile-first).
> - Crie `style/_layout.scss` com o grid/containers principais e o cabeçalho/rodapé do site usando logo e identidade visual reais.
> - Configure fontes (locais em `public/`, sem CDN externo por segurança/performance).
> Resultado: um layout base vazio porém já com a identidade visual do DRINKUP aplicada.

---

## Fase 2 — Banco de dados e migrations

> Modele o schema do catálogo em migrations SQLx (`migrations/`):
> - `categorias` (de copos), `produtos` (nome, descrição, slug, categoria, atributos: capacidade/ml, material, cor, medidas, etc.), `produto_imagens`.
> - `usuarios` (do painel) com `papel` (admin/editor/gerente), hash de senha, status.
> - `orcamentos`/`contatos` (lead do cliente: nome, contato, mensagem, itens de interesse).
> - `sessions` (store do tower-sessions) e `audit_log` (ações administrativas).
> Adicione índices para busca/filtro e seeds mínimos de desenvolvimento. Configure conexão SQLx (`src/server/db.rs`) com pool. Use `sqlx::query!` (checagem em compilação) e prepare o modo offline.

---

## Fase 3 — Domínio compartilhado + esqueleto de server functions

> - Em `src/domain/`, defina os tipos compartilhados cliente/servidor (DTOs de produto/categoria para exibição) — **sem** campos sensíveis e **sem** lógica de negócio.
> - Em `src/server/`, crie o esqueleto das server functions (listar produtos, detalhe de produto, enviar orçamento) com a estrutura de erro segura (`src/error.rs`) e o helper de RBAC (`src/server/rbac.rs`) — ainda que stubs.
> - Reforce o limite: nada de `src/server/` é exposto ao bundle `hydrate`.

---

## Fase 4 — Vitrine pública (catálogo)

> Implemente as páginas públicas (SSR), seguindo os mockups de `assets/design/`:
> - Home (destaques, identidade visual, chamada para orçamento).
> - Listagem de produtos com **busca, filtros (categoria, material, cor, capacidade) e paginação** — toda a filtragem/paginação no servidor.
> - Página de detalhe do produto (galeria de imagens, atributos, botão "Pedir orçamento").
> - Componentes reutilizáveis: `ProductCard`, `Gallery`, `FilterBar`, `Pagination`, `Header`, `Footer`.
> Componentes são "burros": recebem dados já resolvidos pelas server functions e apenas renderizam. Estados de carregando/vazio/erro implementados.

---

## Fase 5 — Fluxo de orçamento / contato

> - Formulário de orçamento (a partir de um produto ou geral): nome, contato, itens de interesse, mensagem.
> - Validação no cliente (UX) **e** revalidação completa no servidor.
> - Server function que persiste o lead em `orcamentos` e dispara notificação (ex.: e-mail/registro) — sem segredos no cliente.
> - Proteção anti-spam (rate limit + honeypot/checagem simples). Mensagens de sucesso/erro genéricas e amigáveis.

---

## Fase 6 — Autenticação e RBAC

> Implemente o login do painel com **toda a autorização no servidor**:
> - Hash de senha com Argon2id; verificação em server function.
> - Sessão via `tower-sessions` (store Postgres), cookie `HttpOnly`/`Secure`/`SameSite`, expiração.
> - Rate limiting no login + mensagem de falha genérica.
> - Helper de RBAC: cada server function protegida declara o papel mínimo; rota do admin redireciona não autenticados.
> - Registro em `audit_log` de login/logout e ações sensíveis.

---

## Fase 7 — Painel administrativo

> Construa o painel em `src/admin/` seguindo o mockup do painel em `assets/design/`:
> - Dashboard (resumos: nº de produtos, orçamentos recentes).
> - CRUD de categorias e produtos, com **upload de imagens** validado (tipo/tamanho reais) e servido de caminho controlado.
> - Gestão de orçamentos/leads (listar, ver, marcar status).
> - Gestão de usuários e papéis (somente `admin`).
> Toda ação passa por server function com checagem de papel. UI consistente com os componentes de design. Tabelas com paginação e busca server-side.

---

## Fase 8 — Endurecimento de segurança

> Revise e implemente as defesas da seção 3 do CLAUDE.md:
> - Headers de segurança (CSP, HSTS em prod, X-Content-Type-Options, Referrer-Policy).
> - CSRF nas server functions mutáveis.
> - Auditoria de que nenhum segredo/lógica sensível foi para o bundle `hydrate`.
> - Verificação de que todas as server functions validam entrada e autorização.
> - Sanitização/escape de qualquer conteúdo dinâmico renderizado.
> Rode o /security-review nas mudanças e corrija os achados.

---

## Fase 9 — SEO e desempenho

> - Meta tags por página (título, descrição, Open Graph), `sitemap.xml`, `robots.txt`, dados estruturados de produto.
> - Otimização de imagens (tamanhos responsivos, lazy load, cache com hash no nome).
> - Build `--release`, bundle WASM enxuto, headers de cache para assets estáticos.
> - Conferir LCP/CLS e tamanho do bundle; ajustar hidratação para o mínimo necessário.

---

## Fase 10 — Deploy (VPS / Docker)

> - `Dockerfile` multi-stage (build com cargo-leptos → imagem final mínima).
> - `docker-compose.yml` de produção (app + Postgres + volume + rede), variáveis via ambiente (sem segredos no repo).
> - Rodar migrations no deploy. Healthcheck. Logs estruturados.
> - Documentar o procedimento de deploy em `docs/`.

---

## Como usar este arquivo

- Execute uma fase por vez; só avance quando a anterior estiver funcionando, `fmt`/`clippy` limpos.
- Ao final de cada fase, valide rodando o app (`cargo leptos watch`).
- Se algo no escopo mudar, atualize o `CLAUDE.md` e este roteiro juntos.
