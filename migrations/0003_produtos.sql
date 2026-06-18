-- Produtos (copos) e suas imagens.
CREATE TABLE produtos (
    id             uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    categoria_id   uuid REFERENCES categorias (id) ON DELETE SET NULL,
    nome           text NOT NULL,
    slug           text NOT NULL UNIQUE,
    descricao      text,
    capacidade_ml  integer,            -- volume em ml
    material       text,               -- ex.: "Acrílico", "PS cristal"
    cor            text,
    altura_mm      integer,
    diametro_mm    integer,
    personalizavel boolean NOT NULL DEFAULT true,
    destaque       boolean NOT NULL DEFAULT false,
    ativo          boolean NOT NULL DEFAULT true,
    visualizacoes  bigint  NOT NULL DEFAULT 0,
    created_at     timestamptz NOT NULL DEFAULT now(),
    updated_at     timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_produtos_categoria ON produtos (categoria_id);
CREATE INDEX idx_produtos_ativo_destaque ON produtos (ativo, destaque);
CREATE INDEX idx_produtos_capacidade ON produtos (capacidade_ml);
-- Busca por nome (case-insensitive / fuzzy) acelerada por trigramas.
CREATE INDEX idx_produtos_nome_trgm ON produtos USING gin (lower(nome) gin_trgm_ops);

CREATE TRIGGER trg_produtos_updated
    BEFORE UPDATE ON produtos
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TABLE produto_imagens (
    id         uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    produto_id uuid NOT NULL REFERENCES produtos (id) ON DELETE CASCADE,
    url        text NOT NULL,
    alt        text,
    ordem      integer NOT NULL DEFAULT 0,
    principal  boolean NOT NULL DEFAULT false,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_produto_imagens_produto ON produto_imagens (produto_id, ordem);
-- No máximo uma imagem principal por produto.
CREATE UNIQUE INDEX uq_produto_imagem_principal
    ON produto_imagens (produto_id) WHERE principal;
