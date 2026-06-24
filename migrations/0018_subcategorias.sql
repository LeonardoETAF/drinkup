-- Subcategorias: uma subcategoria é uma linha em `categorias` com `parent_id`
-- apontando para uma categoria de nível superior (hierarquia de 2 níveis).
-- Excluir a categoria-pai remove suas subcategorias (CASCADE).
ALTER TABLE categorias
    ADD COLUMN parent_id uuid REFERENCES categorias (id) ON DELETE CASCADE;

CREATE INDEX idx_categorias_parent ON categorias (parent_id);

-- Produto pode ter uma subcategoria (opcional), além da categoria. Remover a
-- subcategoria apenas desvincula o produto (SET NULL).
ALTER TABLE produtos
    ADD COLUMN subcategoria_id uuid REFERENCES categorias (id) ON DELETE SET NULL;

CREATE INDEX idx_produtos_subcategoria ON produtos (subcategoria_id);
