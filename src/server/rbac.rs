//! Controle de acesso por papel (RBAC). A verificação real (extrair o usuário
//! da sessão e barrar o acesso) é implementada na Fase 6; aqui ficam o tipo de
//! papel e a hierarquia usada pelas server functions protegidas.

/// Menus do painel sujeitos a permissão por usuário.
pub const MENUS: [&str; 8] = [
    "dashboard",
    "produtos",
    "leads",
    "parceiros",
    "eventos",
    "conteudo",
    "quem-somos",
    "configuracoes",
];

/// Papéis do painel, do mais para o menos privilegiado.
/// `Visualizador` só pode ver (não cria/edita/exclui nada).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Papel {
    Admin,
    Gerente,
    Editor,
    Visualizador,
}

impl Papel {
    /// Converte do valor armazenado no banco (coluna `usuarios.papel`).
    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(Self::Admin),
            "gerente" => Some(Self::Gerente),
            "editor" => Some(Self::Editor),
            "visualizador" => Some(Self::Visualizador),
            _ => None,
        }
    }

    /// Nível numérico (maior = mais privilégio).
    fn nivel(self) -> u8 {
        match self {
            Self::Admin => 3,
            Self::Gerente => 2,
            Self::Editor => 1,
            Self::Visualizador => 0,
        }
    }

    /// `true` se este papel atende ao papel mínimo exigido.
    pub fn atende(self, minimo: Papel) -> bool {
        self.nivel() >= minimo.nivel()
    }
}
