use leptos::prelude::*;

use crate::domain::HomeForm;

/// Carrega o conteúdo da home para edição (papel mínimo: gerente).
#[server]
pub async fn obter_home_form() -> Result<HomeForm, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Gerente).await?;
    crate::server::home_content::obter_form(&pool)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível carregar o conteúdo."))
}

/// Grava o conteúdo da home (papel mínimo: gerente).
#[server]
pub async fn salvar_home(form: HomeForm) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Gerente).await?;
    crate::server::home_content::salvar(&pool, &form)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível salvar o conteúdo."))
}
