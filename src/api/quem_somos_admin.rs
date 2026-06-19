use leptos::prelude::*;

use crate::domain::QuemSomosForm;

/// Carrega o conteúdo de "Quem Somos" para edição (papel mínimo: gerente).
#[server]
pub async fn obter_quem_somos_form() -> Result<QuemSomosForm, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Visualizador, "quem-somos").await?;
    crate::server::quem_somos_content::obter_form(&pool)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível carregar o conteúdo."))
}

/// Grava o conteúdo de "Quem Somos" (papel mínimo: gerente).
#[server]
pub async fn salvar_quem_somos(form: QuemSomosForm) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Gerente, "quem-somos").await?;
    crate::server::quem_somos_content::salvar(&pool, &form)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível salvar o conteúdo."))
}
