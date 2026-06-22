//! Páginas públicas (vitrine).
mod contato;
mod home;
mod parceiros;
mod privacidade;
mod produto;
mod produtos;
mod sobre;
mod termos;

pub use contato::ContatoPage;
pub use home::HomePage;
pub use parceiros::ParceirosPage;
pub use privacidade::PrivacidadePage;
pub use produto::ProdutoPage;
pub use produtos::ProdutosPage;
pub use sobre::QuemSomosPage;
pub use termos::TermosPage;
