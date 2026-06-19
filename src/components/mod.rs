//! Componentes de UI reutilizáveis e "burros": recebem dados já resolvidos pelas
//! server functions e apenas renderizam. Sem lógica de negócio nem segredos.
mod filter_bar;
mod gallery;
mod imagem;
mod page_hero;
mod pagination;
pub(crate) mod product_card;
mod seo;
mod site_footer;
mod site_header;
mod tema;

pub use filter_bar::FilterBar;
pub use gallery::Gallery;
pub use imagem::responsiva;
pub use page_hero::PageHero;
pub use pagination::Pagination;
pub use product_card::ProductCard;
pub use seo::{Seo, SITE_URL};
pub use site_footer::SiteFooter;
pub use site_header::SiteHeader;
pub use tema::BotaoTema;
