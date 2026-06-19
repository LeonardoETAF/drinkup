-- Inscritos em "Novidades" (newsletter por WhatsApp), captados no rodapé do site.
CREATE TABLE novidades_inscritos (
    id         uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    telefone   text NOT NULL UNIQUE,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_novidades_created ON novidades_inscritos (created_at DESC);

-- Concede o novo menu "novidades" a quem já administra contatos (tem 'leads').
UPDATE usuarios
   SET menus = array_append(menus, 'novidades')
 WHERE 'leads' = ANY (menus) AND NOT ('novidades' = ANY (menus));
