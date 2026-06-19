use leptos::prelude::*;

use crate::domain::QuemSomosConteudo;

/// Conteúdo editável da página "Quem Somos". Público.
#[server]
pub async fn obter_quem_somos() -> Result<QuemSomosConteudo, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::server::quem_somos_content::obter(&pool)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível carregar o conteúdo."))
}
