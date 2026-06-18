# Deploy (VPS + Docker)

Stack: imagem multi-stage do app + PostgreSQL via `docker-compose.prod.yml`,
atrás de um proxy reverso (TLS/HTTPS + rate-limit).

## 1. Pré-requisitos no servidor
- Docker + plugin `docker compose`.
- Um domínio apontando para o IP do VPS (DNS A/AAAA). O site usa
  `https://drinkup.com.br` (definido em `src/components/seo.rs` → `SITE_URL`;
  troque ali e rebuilde se o domínio mudar).

## 2. Configuração
```bash
cp .env.example .env
# Edite e defina, no mínimo:
#   POSTGRES_PASSWORD  -> senha forte e única
#   SESSION_SECRET     -> openssl rand -base64 64
```
`DATABASE_URL`, `LEPTOS_SITE_ADDR`, `LEPTOS_ENV=PROD` e os caminhos de assets já
são resolvidos pela imagem/compose em produção — não precisa defini-los.

## 3. Subir
```bash
docker compose -f docker-compose.prod.yml up -d --build
docker compose -f docker-compose.prod.yml logs -f app   # acompanhar
```
- As **migrations rodam no startup** (embutidas no binário).
- O app sobe em `127.0.0.1:3000` (somente loopback) — o proxy expõe à internet.
- Uploads persistem no volume `drinkup_uploads`; o banco em `drinkup_db_data`.

### Primeiro usuário admin
O seed de desenvolvimento NÃO é usado em produção. Crie o primeiro admin com
hash Argon2id (gere o hash uma vez e insira no banco):
```bash
docker compose -f docker-compose.prod.yml exec db \
  psql -U drinkup -d drinkup -c \
  "INSERT INTO usuarios (nome,email,senha_hash,papel,ativo)
   VALUES ('Admin','voce@exemplo.com','<HASH_ARGON2>','admin',true);"
```
> Dica: rode o app localmente e use a tela de Usuários (ou um pequeno binário
> que chame `server::auth::gerar_hash`) para produzir o `<HASH_ARGON2>`.

## 4. Proxy reverso (Nginx) — TLS + rate-limit por IP

TLS via Let's Encrypt (certbot) e **rate-limit por IP no login** (defesa que
complementa o limite por e-mail já existente na aplicação):

```nginx
# /etc/nginx/conf.d/drinkup.conf
limit_req_zone $binary_remote_addr zone=login:10m rate=5r/m;

server {
    listen 443 ssl http2;
    server_name drinkup.com.br;

    ssl_certificate     /etc/letsencrypt/live/drinkup.com.br/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/drinkup.com.br/privkey.pem;

    client_max_body_size 6m;            # alinhado ao DefaultBodyLimit do app

    # Rate-limit nas chamadas de login (server function).
    location /api/login {
        limit_req zone=login burst=5 nodelay;
        proxy_pass http://127.0.0.1:3000;
        include proxy_params;
    }

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host              $host;
        proxy_set_header X-Real-IP         $remote_addr;
        proxy_set_header X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}

server {                                 # redireciona HTTP -> HTTPS
    listen 80;
    server_name drinkup.com.br;
    return 301 https://$host$request_uri;
}
```
HSTS, CSP, X-Frame-Options etc. já vêm do app (Fase 8) — não duplicar no Nginx.

## 5. Operação
```bash
# Atualizar para uma nova versão
git pull && docker compose -f docker-compose.prod.yml up -d --build

# Backup do banco
docker compose -f docker-compose.prod.yml exec db \
  pg_dump -U drinkup drinkup | gzip > backup-$(date +%F).sql.gz

# Backup dos uploads (o Compose prefixa o volume com o nome do projeto/pasta;
# confirme o nome real com `docker volume ls`)
VOL=$(docker volume ls -q | grep drinkup_prod_uploads | head -1)
docker run --rm -v "$VOL":/u -v "$PWD":/b alpine \
  tar czf /b/uploads-$(date +%F).tar.gz -C /u .
```

## Checklist de segurança em produção
- [ ] `SESSION_SECRET` forte e único; `POSTGRES_PASSWORD` forte.
- [ ] HTTPS válido (cookies são `Secure` em release — sem HTTPS o login não persiste).
- [ ] Porta do Postgres NÃO publicada para a internet (o compose de prod não a publica).
- [ ] Backups de banco e uploads agendados.
- [ ] Rate-limit do `/api/login` ativo no proxy.
