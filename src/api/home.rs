use leptos::prelude::*;

use crate::domain::HomeConteudo;

/// Conteúdo editável da home (faixa de números + bento). Público.
#[server]
pub async fn obter_home_conteudo() -> Result<HomeConteudo, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::server::home_content::obter(&pool)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível carregar o conteúdo."))
}
