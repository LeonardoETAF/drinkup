-- Categorias de copos.
CREATE TABLE categorias (
    id         uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    nome       text NOT NULL,
    slug       text NOT NULL UNIQUE,
    descricao  text,
    ordem      integer NOT NULL DEFAULT 0,
    ativo      boolean NOT NULL DEFAULT true,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_categorias_ativo_ordem ON categorias (ativo, ordem);

CREATE TRIGGER trg_categorias_updated
    BEFORE UPDATE ON categorias
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
