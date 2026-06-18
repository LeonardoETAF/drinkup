-- Tokens de redefinição de senha (uso único, com expiração).
-- O token em si NUNCA é armazenado; guardamos apenas seu hash (SHA-256).
CREATE TABLE password_resets (
    id         uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    usuario_id uuid NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
    token_hash text NOT NULL,
    expira_em  timestamptz NOT NULL,
    usado_em   timestamptz,
    criado_em  timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_password_resets_token ON password_resets (token_hash);
CREATE INDEX idx_password_resets_usuario ON password_resets (usuario_id);
