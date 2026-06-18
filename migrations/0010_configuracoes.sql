-- Configurações da loja (chave/valor) editáveis no painel.
CREATE TABLE configuracoes (
    chave text PRIMARY KEY,
    valor text NOT NULL DEFAULT ''
);
