use leptos::prelude::*;
use uuid::Uuid;

use crate::domain::PaginaInscritos;

/// Inscreve um WhatsApp em "Novidades" (público). Validação/dedupe no servidor.
#[server]
pub async fn inscrever_novidades(telefone: String) -> Result<(), ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    match crate::server::novidades::inscrever(&pool, &telefone).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => {
            Err(ServerFnError::new("Informe um número de WhatsApp válido."))
        }
        Err(_) => Err(ServerFnError::new("Não foi possível concluir a inscrição.")),
    }
}

/// Lista os inscritos (admin). Exige autenticação (papel: visualizador).
#[server]
pub async fn listar_inscritos(
    busca: Option<String>,
    pagina: u32,
) -> Result<PaginaInscritos, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Visualizador, "novidades").await?;
    crate::server::novidades::listar(
        &pool,
        busca.as_deref(),
        i64::from(pagina.max(1)),
        crate::domain::ADMIN_TABELA_POR_PAGINA,
    )
    .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar inscritos");
            ServerFnError::new("Não foi possível carregar os inscritos.")
        })
}

/// Atualiza a classificação de um inscrito (admin). Exige papel: editor.
#[server]
pub async fn classificar_inscrito(id: Uuid, classificacao: String) -> Result<(), ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Editor, "novidades").await?;
    match crate::server::novidades::atualizar_classificacao(&pool, id, &classificacao).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => Err(ServerFnError::new("Classificação inválida.")),
        Err(_) => Err(ServerFnError::new("Não foi possível atualizar.")),
    }
}

/// Remove um inscrito (admin). Exige autenticação (papel: editor).
#[server]
pub async fn excluir_inscrito(id: Uuid) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Editor, "novidades").await?;
    crate::server::novidades::excluir(&pool, id)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível remover o inscrito."))
}
