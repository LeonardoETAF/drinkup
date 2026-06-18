-- Usuários do painel administrativo (RBAC). Senha: Argon2id (definido na Fase 6).
CREATE TABLE usuarios (
    id           uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    nome         text NOT NULL,
    email        text NOT NULL,
    senha_hash   text NOT NULL,
    papel        text NOT NULL DEFAULT 'editor'
                   CHECK (papel IN ('admin', 'gerente', 'editor')),
    ativo        boolean NOT NULL DEFAULT true,
    ultimo_login timestamptz,
    created_at   timestamptz NOT NULL DEFAULT now(),
    updated_at   timestamptz NOT NULL DEFAULT now()
);

-- E-mail único, case-insensitive.
CREATE UNIQUE INDEX uq_usuarios_email_lower ON usuarios (lower(email));
CREATE INDEX idx_usuarios_papel ON usuarios (papel);

CREATE TRIGGER trg_usuarios_updated
    BEFORE UPDATE ON usuarios
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
