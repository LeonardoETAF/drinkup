-- Permite o novo papel "visualizador" (somente leitura).
ALTER TABLE usuarios DROP CONSTRAINT usuarios_papel_check;
ALTER TABLE usuarios ADD CONSTRAINT usuarios_papel_check
    CHECK (papel IN ('admin', 'gerente', 'editor', 'visualizador'));
