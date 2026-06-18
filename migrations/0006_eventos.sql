-- Eventos (portfólio: formaturas, casamentos, corporativos, etc.).
CREATE TABLE eventos (
    id          uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    titulo      text NOT NULL,
    slug        text NOT NULL UNIQUE,
    descricao   text,
    tipo        text NOT NULL DEFAULT 'outro'
                  CHECK (tipo IN ('formatura', 'casamento', 'corporativo', 'aniversario', 'outro')),
    data_evento date,
    local       text,
    imagem_url  text,
    destaque    boolean NOT NULL DEFAULT false,
    ativo       boolean NOT NULL DEFAULT true,
    created_at  timestamptz NOT NULL DEFAULT now(),
    updated_at  timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_eventos_ativo_data ON eventos (ativo, data_evento DESC);

CREATE TRIGGER trg_eventos_updated
    BEFORE UPDATE ON eventos
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
