-- Parceiros (logos exibidos no site e gerenciados no painel).
CREATE TABLE parceiros (
    id         uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    nome       text NOT NULL,
    slug       text NOT NULL UNIQUE,
    logo_url   text,
    site_url   text,
    descricao  text,
    ordem      integer NOT NULL DEFAULT 0,
    ativo      boolean NOT NULL DEFAULT true,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_parceiros_ativo_ordem ON parceiros (ativo, ordem);

CREATE TRIGGER trg_parceiros_updated
    BEFORE UPDATE ON parceiros
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
