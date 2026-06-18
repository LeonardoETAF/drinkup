-- Leads / orçamentos (contato do cliente) e seus itens de interesse.
CREATE TABLE leads (
    id         uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    nome       text NOT NULL,
    contato    text NOT NULL,          -- WhatsApp / telefone
    email      text,
    origem     text NOT NULL DEFAULT 'site'
                 CHECK (origem IN ('site', 'instagram', 'facebook', 'google', 'whatsapp', 'indicacao', 'outro')),
    mensagem   text,
    status     text NOT NULL DEFAULT 'novo'
                 CHECK (status IN ('novo', 'em_atendimento', 'convertido', 'perdido')),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_leads_status ON leads (status, created_at DESC);
CREATE INDEX idx_leads_origem ON leads (origem);

CREATE TRIGGER trg_leads_updated
    BEFORE UPDATE ON leads
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TABLE lead_itens (
    id         uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    lead_id    uuid NOT NULL REFERENCES leads (id) ON DELETE CASCADE,
    produto_id uuid REFERENCES produtos (id) ON DELETE SET NULL,
    descricao  text,                   -- item livre quando não há produto associado
    quantidade integer NOT NULL DEFAULT 1 CHECK (quantidade > 0)
);

CREATE INDEX idx_lead_itens_lead ON lead_itens (lead_id);
CREATE INDEX idx_lead_itens_produto ON lead_itens (produto_id);
