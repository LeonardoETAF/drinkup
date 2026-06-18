-- Log de auditoria de ações administrativas (append-only).
CREATE TABLE audit_log (
    id          bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    usuario_id  uuid REFERENCES usuarios (id) ON DELETE SET NULL,
    acao        text NOT NULL,          -- ex.: "produto.criar"
    entidade    text,                   -- ex.: "produto"
    entidade_id text,
    detalhes    jsonb,
    ip          inet,
    created_at  timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_audit_log_usuario ON audit_log (usuario_id, created_at DESC);
CREATE INDEX idx_audit_log_entidade ON audit_log (entidade, entidade_id);

-- NOTA: a tabela de sessões (tower-sessions) é criada/migrada na Fase 6,
-- pela própria store do tower-sessions (esquema gerido por ela).
