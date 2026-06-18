//! Painel administrativo (rotas protegidas). A autorização é SEMPRE verificada
//! no servidor (sessão + RBAC); a UI aqui é apresentação, não controle de acesso.
mod dashboard;
mod em_construcao;
mod layout;
mod leads;
mod login;
mod recuperar;
mod util;

pub use dashboard::AdminDashboard;
pub use em_construcao::AdminEmBreve;
pub use layout::AdminLayout;
pub use leads::AdminLeads;
pub use login::AdminLoginPage;
pub use recuperar::AdminRecuperarSenhaPage;
