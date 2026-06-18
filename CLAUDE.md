# DRINKUP — Regras do Projeto

> Catálogo de copos de plástico. Vitrine pública + painel administrativo.
> 100% Rust. Foco: **UI/UX moderna e bonita**, **segurança** e **desempenho**.
> Este arquivo é a fonte de verdade do projeto. Toda decisão de código deve respeitá-lo.

---

## 1. Stack (fixa — não trocar sem registrar aqui)

| Camada | Tecnologia |
|--------|-----------|
| Linguagem | Rust (edition 2021, toolchain estável) |
| Frontend | Leptos 0.8.x (SSR + hidratação) |
| Backend | Axum (via `leptos_axum`) + server functions |
| Banco | PostgreSQL + SQLx (queries verificadas em compilação) |
| Migrations | `sqlx-cli` (`migrations/`) |
| Sessão/Auth | `tower-sessions` + store Postgres + `argon2` (hash de senha). RBAC próprio. |
| Estilização | CSS puro + variáveis CSS (design tokens), em parciais SCSS usados só para concatenar num único arquivo. **Sem Node, sem Tailwind.** |
| Build | `cargo-leptos` (0.3.x) |
| Deploy | VPS próprio, Docker (`docker-compose`) + Postgres |
| Idioma | PT-BR (sem i18n por enquanto) |

Versões exatas são fixadas no `Cargo.toml` no scaffold e o `Cargo.lock` é commitado.

---

## 2. Princípio central de segurança: **FRONTEND BURRO**

O cliente (WASM/HTML no navegador) é **descartável e não confiável**. Regras inegociáveis:

1. **Zero segredos no cliente.** Nenhuma chave, credencial, connection string ou variável sensível pode chegar ao bundle WASM. Segredos só via env no servidor.
2. **Autorização SEMPRE no servidor.** Toda server function verifica sessão + papel antes de qualquer ação. Esconder um botão na UI **não** é controle de acesso.
3. **Validação real no servidor.** Validação no cliente existe só para UX (feedback rápido). O servidor revalida 100% das entradas e nunca confia no cliente.
4. **Server functions são a única porta de dados.** O cliente não acessa o banco, não tem lógica de negócio sensível, não calcula nada que importe. Ele chama, recebe, renderiza.
5. **Dados sensíveis nunca trafegam ao cliente.** Custos internos, dados de outros usuários, campos administrativos — só vão ao cliente quem tem direito a vê-los, decidido no servidor.
6. **Erros não vazam interno.** Mensagens de erro ao cliente são genéricas; detalhes (stack, SQL, etc.) só em log do servidor.

Se uma feature exige lógica no cliente, pergunte: "isso pode ser forjado?". Se sim, a decisão final é do servidor.

---

## 3. Outras regras de segurança

- **SQL**: usar macros `sqlx::query!`/`query_as!` (prepared statements, checadas em compilação). Nunca concatenar SQL com input. Habilitar `SQLX_OFFLINE` + `cargo sqlx prepare` para build sem banco.
- **Senhas**: `argon2` (Argon2id). Nunca armazenar/loggar senha em texto.
- **Sessão**: cookie `HttpOnly`, `Secure`, `SameSite=Lax/Strict`, expiração definida. Sessão server-side em Postgres.
- **Login**: rate limiting + proteção contra brute force. Mensagem de falha genérica ("credenciais inválidas").
- **CSRF**: server functions mutáveis protegidas (token/origin check).
- **Headers**: CSP restritiva, `X-Content-Type-Options`, `Referrer-Policy`, HSTS em produção.
- **Uploads** (imagens de produto): validar tipo/tamanho real, renomear, servir de caminho controlado, nunca executar.
- **RBAC**: papéis (ex.: `admin`, `editor`, `gerente`). Cada server function declara o papel mínimo exigido. Centralizar a checagem num helper.
- **Logs**: sem PII sensível nem segredos. Auditar ações administrativas (quem fez o quê, quando).

---

## 4. Desempenho

- Preferir SSR para a vitrine (HTML pronto = bom LCP e SEO). Hidratar só o necessário.
- Imagens otimizadas (tamanhos responsivos, `loading="lazy"`, formatos modernos), servidas com cache longo + hash no nome.
- Bundle WASM enxuto: build em `--release`, `opt-level` adequado, evitar dependências pesadas no cliente.
- Banco: índices nas colunas de busca/filtro; paginação em todas as listagens; evitar N+1.
- Cache de respostas estáticas e assets via headers.

---

## 5. UI/UX

- Visual moderno, limpo e bonito é requisito, não enfeite. Seguir os **materiais de design** fornecidos (mockups do site e do painel).
- **Design tokens em CSS** (`style/_tokens.scss`): cores, tipografia, espaçamentos, raios, sombras, breakpoints — extraídos do material de design. Nada de valores mágicos espalhados. O conteúdo é CSS puro + variáveis CSS; o SCSS só junta os parciais (o cargo-leptos não empacota `@import` de `.css`).
- Componentes de UI reutilizáveis e consistentes (botões, cards, inputs, modais, tabelas).
- **Responsivo e adaptativo por completo (obrigatório).** TODA tela — site público e painel admin — deve funcionar e ficar bonita em qualquer tamanho: do celular pequeno (~320px) ao desktop largo, passando por tablet. Mobile-first. Nada de scroll horizontal, corte de conteúdo ou elementos sobrepostos em nenhum breakpoint.
  - Layouts fluidos (grid/flex, unidades relativas, `clamp()`); evitar larguras/alturas fixas em px que quebrem.
  - Padrões adaptativos onde necessário: menu vira hambúrguer, tabelas viram cards/rolagem controlada, colunas reempilham, imagens responsivas (`srcset`/tamanhos).
  - Considerar toque e mouse (alvos de toque ≥ 44px, estados de hover/focus equivalentes) e respeitar `prefers-reduced-motion`.
  - Toda entrega de UI só é considerada pronta após validada em mobile, tablet e desktop.
- Acessibilidade: contraste, foco visível, labels, navegação por teclado, `alt` em imagens.
- Estados claros: carregando, vazio, erro, sucesso.

---

## 6. Estrutura de pastas (alvo)

```
drinkup/
├── Cargo.toml              # crate único, features `ssr` e `hydrate`
├── Cargo.lock              # commitado
├── rust-toolchain.toml
├── .env.example            # nunca commitar .env real
├── .gitignore
├── docker-compose.yml      # app + postgres
├── Dockerfile              # multi-stage
├── CLAUDE.md               # este arquivo
├── docs/
│   └── PROMPTS.md          # roteiro de desenvolvimento (prompts)
├── migrations/             # SQLx migrations versionadas
├── seeds/                  # seeds de desenvolvimento (não-produção)
├── .sqlx/                  # cache de queries p/ build offline (commitado)
├── public/                 # servido como está (favicon, robots.txt, imagens públicas)
├── assets/                 # MATERIAIS DE DESIGN (fonte)
│   ├── logos/
│   ├── icons/
│   ├── backgrounds/
│   └── design/             # mockups do site e do painel (referência visual)
├── style/
│   ├── main.scss           # entrada (só @import dos parciais)
│   ├── _tokens.scss        # variáveis CSS / design tokens
│   ├── _base.scss          # reset + tipografia base
│   ├── _layout.scss
│   └── components/         # parciais por componente (_card.scss, ...)
└── src/
    ├── main.rs             # entrada do servidor (feature ssr) — #![forbid(unsafe_code)]
    ├── lib.rs              # reexports + módulos — #![deny(unsafe_code)]
    ├── hydration.rs        # bootstrap de hidratação (único allow de unsafe: wasm-bindgen)
    ├── app.rs              # componente raiz + rotas
    ├── api/                # server functions (#[server]) — boundary cliente/servidor
    ├── components/         # UI reutilizável (burra)
    ├── pages/              # páginas públicas (vitrine)
    ├── admin/              # painel (rotas protegidas)
    ├── server/             # APENAS ssr: dados + lógica sensível (chamado pela api/)
    │   ├── db.rs
    │   ├── auth.rs
    │   ├── rbac.rs
    │   ├── products.rs     # repositório (query!/query_as! verificadas em compilação)
    │   └── quotes.rs
    ├── domain/             # tipos compartilhados cliente/servidor (SEM lógica sensível)
    └── error.rs
```

Regra de ouro: tudo dentro de `src/server/` é compilado só com a feature `ssr` e **nunca** vai ao cliente.

---

## 7. Convenções de código

### Arquitetura modular e responsabilidade única
- **Estrutura modular obrigatória.** Cada módulo/arquivo tem **uma** responsabilidade clara. Separar domínio, acesso a dados, lógica de servidor, UI e tipos compartilhados.
- **Uma responsabilidade por arquivo e por função.** Se um arquivo faz duas coisas, divida em dois módulos. Se uma função decide, valida e persiste, quebre em funções menores e compostas.
- **Arquivos e funções pequenos.** Diretriz (não dogma): arquivo ~300 linhas, função ~50 linhas. Passou bastante disso → sinal forte de que precisa ser dividido. Preferir muitos arquivos pequenos e coesos a poucos arquivos grandes.
- Baixo acoplamento, alta coesão. Dependências fluem do servidor → domínio (compartilhado), nunca o contrário. UI depende de `domain`, não de `server`.
- Reaproveitar via componentes/funções/módulos em vez de duplicar.

### Boas práticas de Rust
- **Proibido `unsafe`.** Nenhum bloco `unsafe` no nosso código. Binário do servidor: `#![forbid(unsafe_code)]`. Lib: `#![deny(unsafe_code)]`. Única exceção: o ponto de entrada de hidratação (`src/hydration.rs`), cujo `unsafe` é gerado pelo `wasm-bindgen` (glue do framework) — isolado e com `#![allow(unsafe_code)]` apenas naquele módulo.
- Seguir as boas práticas idiomáticas do Rust (API Guidelines): ownership/borrowing claros, `Result`/`Option` em vez de sentinelas, `?` para propagar erro, iteradores em vez de loops manuais quando mais legível, tipos fortes em vez de primitivos soltos (newtypes quando fizer sentido).
- `rustfmt` + `clippy` limpos antes de concluir qualquer tarefa. `clippy` tratado como erro no que for razoável (`-D warnings` no que couber).
- Sem `unwrap()`/`expect()` em caminho de produção do servidor (só em testes/inicialização controlada). Sem `panic!` em fluxo normal.
- Erros: `thiserror` no domínio/servidor; converter para erro genérico/seguro na borda com o cliente.
- Visibilidade mínima: `pub` só no necessário; preferir `pub(crate)`/privado.
- `#[must_use]` em retornos relevantes; evitar `clone()` desnecessário; emprestar quando possível.

### Geral
- Nomes em inglês no código; textos de UI em PT-BR.
- Funções pequenas, puras quando possível e testáveis. Lógica de negócio testada (testes no servidor).
- Comentários só quando agregam; código deve ser legível por si.

---

## 8. Comandos

```bash
# Desenvolvimento (hot reload)
cargo leptos watch

# Build de produção
cargo leptos build --release

# Migrations
sqlx migrate add <nome>
sqlx migrate run

# Preparar queries offline (antes de build sem banco / CI)
cargo sqlx prepare

# Qualidade
cargo fmt
cargo clippy --all-targets

# Subir banco local
docker compose up -d db
```

---

## 9. Variáveis de ambiente (servidor)

Definidas em `.env` (local) e no ambiente do container (produção). Ver `.env.example`.

- `DATABASE_URL` — conexão Postgres
- `LEPTOS_SITE_ADDR` / `LEPTOS_SITE_ROOT` — config Leptos
- `SESSION_SECRET` — chave de assinatura de sessão (forte, secreta)
- `RUST_LOG` — nível de log
- `DRINKUP_DB_PORT` — (só `docker-compose`) porta do host p/ o Postgres; default `5432`. Útil quando a 5432 já está ocupada.

O servidor carrega o `.env` em dev (via `dotenvy`). Nenhuma dessas variáveis pode ser referenciada em código compilado para `hydrate`.

---

## 10. Fluxo de trabalho

- Seguir as fases de `docs/PROMPTS.md` em ordem; não pular fundação.
- Não inventar escopo (ex.: não adicionar e-commerce/pagamento — o escopo é **vitrine + orçamento/contato**).
- Antes de codar uma tela, conferir o mockup correspondente em `assets/design/`.
- Mudou uma decisão de stack/arquitetura? Atualize este arquivo na mesma tarefa.
