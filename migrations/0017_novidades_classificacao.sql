-- Classificação de cada inscrito em "Novidades" (gerenciada no painel).
ALTER TABLE novidades_inscritos
    ADD COLUMN classificacao text NOT NULL DEFAULT 'novo'
        CHECK (classificacao IN ('novo', 'cliente', 'potencial', 'inativo'));
