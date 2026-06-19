-- Permissões de menu por usuário (quais seções do painel ele acessa).
ALTER TABLE usuarios ADD COLUMN menus text[] NOT NULL DEFAULT '{}';

-- Usuários existentes mantêm acesso a tudo.
UPDATE usuarios
   SET menus = ARRAY['dashboard', 'produtos', 'leads', 'parceiros',
                     'eventos', 'conteudo', 'quem-somos', 'configuracoes'];
