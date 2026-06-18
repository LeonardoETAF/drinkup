//! Painel administrativo (rotas protegidas). A autorização é SEMPRE verificada
//! no servidor (sessão + RBAC); a UI aqui é apresentação, não controle de acesso.
mod dashboard;
mod layout;
mod login;

pub use dashboard::AdminDashboard;
pub use layout::AdminLayout;
pub use login::AdminLoginPage;
