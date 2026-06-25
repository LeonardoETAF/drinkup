-- Liga uma visita ao produto visitado (apenas páginas de detalhe de produto).
-- Antes o dashboard cruzava visitas com produtos pelo texto do caminho
-- ('/produtos/' || slug), o que é frágil: renomear o slug órfãava o histórico.
-- Guardar o id torna o ranking correto e dispensa o JOIN por string.
ALTER TABLE visitas
    ADD COLUMN produto_id uuid REFERENCES produtos(id) ON DELETE SET NULL;

-- Backfill do histórico existente pelo caminho atual de cada produto.
UPDATE visitas v
   SET produto_id = p.id
  FROM produtos p
 WHERE v.produto_id IS NULL
   AND v.caminho = '/produtos/' || p.slug;

-- Índice parcial (só visitas de produto) para o agrupamento do ranking.
CREATE INDEX idx_visitas_produto_id ON visitas (produto_id)
    WHERE produto_id IS NOT NULL;
