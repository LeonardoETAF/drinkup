use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Lead em listagem do painel (data já formatada no servidor — sem chrono no cliente).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeadResumo {
    pub id: Uuid,
    pub nome: String,
    pub contato: String,
    pub origem: String,
    pub status: String,
    pub inscricao: String,
}

/// Resumo para o dashboard.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardResumo {
    pub total_produtos: i64,
    pub total_leads: i64,
    pub leads_novos: i64,
    pub total_eventos: i64,
    pub recentes: Vec<LeadResumo>,
}

/// Filtros da listagem de leads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FiltroLeads {
    pub busca: Option<String>,
    pub status: Option<String>,
}

/// Página de leads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginaLeads {
    pub itens: Vec<LeadResumo>,
    pub total: i64,
}

/// Produto na listagem do painel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProdutoLista {
    pub id: Uuid,
    pub nome: String,
    pub categoria: Option<String>,
    pub capacidade_ml: Option<i32>,
    pub ativo: bool,
    pub imagem_url: Option<String>,
}

/// Configurações da loja (editáveis no painel, usadas no site).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuracoes {
    pub nome_loja: String,
    pub cnpj: String,
    pub telefone: String,
    pub endereco: String,
    pub horario_semana: String,
    pub horario_sabado: String,
    pub horario_domingo: String,
}

/// Usuário do painel na listagem.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsuarioLista {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub papel: String,
    pub ativo: bool,
    pub ultimo_login: Option<String>,
}

/// Formulário de usuário do painel. `senha` vazia no editar = manter a atual.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsuarioForm {
    pub id: Option<Uuid>,
    pub nome: String,
    pub email: String,
    pub papel: String,
    pub ativo: bool,
    pub senha: Option<String>,
}

/// Evento = categoria do carrossel da home (listagem do painel).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventoLista {
    pub id: Uuid,
    pub titulo: String,
    pub cor: Option<String>,
    pub ordem: i32,
    pub ativo: bool,
}

/// Formulário de evento/categoria (criar/editar). `id = None` => criação.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventoForm {
    pub id: Option<Uuid>,
    pub titulo: String,
    pub cor: Option<String>,
    pub ordem: i32,
    pub ativo: bool,
}

/// Parceiro na listagem (grade de cards) do painel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParceiroLista {
    pub id: Uuid,
    pub nome: String,
    pub logo_url: Option<String>,
    pub ativo: bool,
}

/// Formulário de parceiro (criar/editar). `id = None` => criação.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParceiroForm {
    pub id: Option<Uuid>,
    pub nome: String,
    pub logo_url: Option<String>,
    pub site_url: Option<String>,
    pub descricao: Option<String>,
    pub ordem: i32,
    pub ativo: bool,
}

/// Formulário de produto (criar/editar). `id = None` => criação.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProdutoForm {
    pub id: Option<Uuid>,
    pub nome: String,
    pub categoria_id: Option<Uuid>,
    pub descricao: Option<String>,
    pub capacidade_ml: Option<i32>,
    pub material: Option<String>,
    pub cor: Option<String>,
    pub altura_mm: Option<i32>,
    pub diametro_mm: Option<i32>,
    pub personalizavel: bool,
    pub destaque: bool,
    pub ativo: bool,
    pub imagem_url: Option<String>,
}
