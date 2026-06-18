-- "Eventos" no painel = categorias do carrossel da home (nome + cor + ordem + visível).
-- Reaproveita a tabela `eventos`: título = nome, ativo = visível.
ALTER TABLE eventos ADD COLUMN cor text;
ALTER TABLE eventos ADD COLUMN ordem integer NOT NULL DEFAULT 0;

CREATE INDEX idx_eventos_ordem ON eventos (ordem);
