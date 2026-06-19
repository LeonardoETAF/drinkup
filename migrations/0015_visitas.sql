-- Rastreio de acessos (page views) das páginas públicas, para o dashboard.
-- Registrado no servidor a cada carregamento de página pública (frontend burro).
CREATE TABLE visitas (
    id         bigserial PRIMARY KEY,
    caminho    text NOT NULL,                       -- rota pública (sem query string)
    origem     text NOT NULL DEFAULT 'direto'       -- de onde veio (referer)
                 CHECK (origem IN ('instagram', 'google', 'facebook', 'whatsapp', 'direto', 'outro')),
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_visitas_created ON visitas (created_at DESC);
CREATE INDEX idx_visitas_caminho ON visitas (caminho);
CREATE INDEX idx_visitas_origem ON visitas (origem);
