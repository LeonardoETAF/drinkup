-- Showcase por marca na página de Parceiros: cor da marca, segmento (tagline)
-- e produtos-exemplo (lista simples de nomes).
ALTER TABLE parceiros ADD COLUMN cor text;
ALTER TABLE parceiros ADD COLUMN tagline text;
ALTER TABLE parceiros ADD COLUMN itens text[] NOT NULL DEFAULT '{}';
