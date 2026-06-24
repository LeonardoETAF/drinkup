-- Medidas adicionais do produto: peso unitário (g) e larguras de base/boca (mm).
-- Todas opcionais. Alturas/larguras seguem armazenadas em mm (exibidas em cm).
ALTER TABLE produtos
    ADD COLUMN peso_g          integer,
    ADD COLUMN largura_base_mm integer,
    ADD COLUMN largura_boca_mm integer;
