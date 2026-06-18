-- Seeds de DESENVOLVIMENTO (não usar em produção). Idempotente.
-- Uso: psql "$DATABASE_URL" -f seeds/dev_seed.sql

-- Categorias
INSERT INTO categorias (nome, slug, descricao, ordem) VALUES
    ('Long Drink', 'long-drink', 'Copos long drink personalizados', 1),
    ('Taças',      'tacas',      'Taças acrílicas para gin e drinks', 2),
    ('Caldereta',  'caldereta',  'Copos caldereta', 3),
    ('Twister',    'twister',    'Copos twister', 4)
ON CONFLICT (slug) DO NOTHING;

-- Produtos (nomes reais do mockup do painel)
INSERT INTO produtos (categoria_id, nome, slug, descricao, capacidade_ml, material, cor, destaque)
SELECT c.id, v.nome, v.slug, v.descricao, v.ml, 'Acrílico', v.cor, v.destaque
FROM (VALUES
    ('long-drink', 'Long Drink 350ml', 'long-drink-350ml', 'Copo long drink 350ml personalizável', 350, 'Transparente', true),
    ('caldereta',  'Caldereta 300ml',  'caldereta-300ml',  'Copo caldereta 300ml',               300, 'Transparente', false),
    ('tacas',      'Taça Gin 580ml',   'taca-gin-580ml',   'Taça gin 580ml',                     580, 'Transparente', true),
    ('twister',    'Twister 300ml',    'twister-300ml',    'Copo twister 300ml',                 300, 'Transparente', false)
) AS v(cat_slug, nome, slug, descricao, ml, cor, destaque)
JOIN categorias c ON c.slug = v.cat_slug
ON CONFLICT (slug) DO NOTHING;

-- Parceiros
INSERT INTO parceiros (nome, slug, ordem) VALUES
    ('Parceiro Exemplo A', 'parceiro-a', 1),
    ('Parceiro Exemplo B', 'parceiro-b', 2)
ON CONFLICT (slug) DO NOTHING;

-- Eventos = categorias do carrossel da home (nome via titulo, cor, ordem, visível via ativo)
INSERT INTO eventos (titulo, slug, cor, ordem) VALUES
    ('Formaturas',   'formaturas',   '#c8d400', 1),
    ('Casamentos',   'casamentos',   '#ff0070', 2),
    ('Aniversários', 'aniversarios', '#00c8ef', 3),
    ('Shows',        'shows',        '#7c3aed', 4),
    ('Baladas',      'baladas',      '#f5821f', 5),
    ('Corporativo',  'corporativo',  '#1f2933', 6),
    ('Restaurantes', 'restaurantes', '#2a9d8f', 7)
ON CONFLICT (slug) DO NOTHING;

-- Configurações da loja (valores padrão)
INSERT INTO configuracoes (chave, valor) VALUES
    ('nome_loja',        'DrinkUp'),
    ('cnpj',             '21.525.492/0001-09'),
    ('telefone',         '(44) 9 9812-4366'),
    ('endereco',         'Rua Rodolfo Cremm, 6436 · Jd. Andrade, Maringá – PR'),
    ('horario_semana',   '8h às 18h'),
    ('horario_sabado',   '8h às 12h'),
    ('horario_domingo',  'Fechado')
ON CONFLICT (chave) DO NOTHING;

-- Usuário admin de desenvolvimento.
-- A senha real (hash Argon2id) é definida na Fase 6; aqui é só um placeholder.
INSERT INTO usuarios (nome, email, senha_hash, papel)
VALUES ('Administrador', 'admin@drinkup.local', 'DEFINIR_NA_FASE_6', 'admin')
ON CONFLICT (lower(email)) DO NOTHING;

-- Um lead de exemplo (só quando ainda não há leads).
WITH novo_lead AS (
    INSERT INTO leads (nome, contato, origem, mensagem, status)
    SELECT 'Mariana Souza', '(44) 99999-0001', 'instagram', 'Quero orçamento para formatura', 'novo'
    WHERE NOT EXISTS (SELECT 1 FROM leads)
    RETURNING id
)
INSERT INTO lead_itens (lead_id, produto_id, quantidade)
SELECT nl.id, p.id, 100
FROM novo_lead nl
JOIN produtos p ON p.slug = 'long-drink-350ml';
