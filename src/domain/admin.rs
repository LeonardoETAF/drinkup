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

/// Uma barra do gráfico de acessos por dia.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiaAcesso {
    pub rotulo: String,
    pub total: i64,
}

/// Fatia da origem do tráfego (com percentual já calculado).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrigemFatia {
    pub origem: String,
    pub total: i64,
    pub pct: i32,
}

/// Linha de "páginas mais visitadas" / "produtos mais vistos".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemRanking {
    pub rotulo: String,
    pub total: i64,
}

/// Resumo para o dashboard (todos os números vêm do banco em tempo real,
/// dentro do período selecionado pelo filtro de ano/mês/dia).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardResumo {
    pub acessos_mes: i64,
    pub acessos_delta: Option<i32>,
    pub total_leads: i64,
    pub leads_delta: Option<i32>,
    pub produtos_total: i64,
    pub produtos_ativos: i64,
    pub total_eventos: i64,
    pub taxa_conversao: f64,
    pub conversao_delta: Option<i32>,
    pub acessos_serie: Vec<DiaAcesso>,
    pub origem_trafego: Vec<OrigemFatia>,
    pub paginas: Vec<ItemRanking>,
    pub produtos_vistos: Vec<ItemRanking>,
    pub recentes: Vec<LeadResumo>,
    // Período efetivamente usado (o servidor resolve o padrão = mês atual).
    pub sel_ano: i32,
    pub sel_mes: Option<i32>,
    pub sel_dia: Option<i32>,
    pub ano_atual: i32,
}

/// Inscrito em "Novidades" (newsletter por WhatsApp).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InscritoResumo {
    pub id: Uuid,
    pub telefone: String,
    pub inscricao: String,
}

/// Página de inscritos em novidades.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginaInscritos {
    pub itens: Vec<InscritoResumo>,
    pub total: i64,
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
    pub facebook: String,
    pub instagram: String,
    pub facebook_ativo: bool,
    pub instagram_ativo: bool,
    pub whatsapp_ativo: bool,
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
    /// Menus que o usuário poderá acessar.
    pub menus: Vec<String>,
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
    pub imagem_url: Option<String>,
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
    pub cor: Option<String>,
    pub tagline: Option<String>,
    pub itens: Vec<String>,
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
    /// URLs das imagens (a primeira é a principal). Exibidas como swipe.
    pub imagens: Vec<String>,
}
