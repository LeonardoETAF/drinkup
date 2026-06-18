use leptos::prelude::*;

use crate::domain::UsuarioSessao;

#[cfg(feature = "ssr")]
const SESSION_UID: &str = "uid";

/// Login do painel. Toda a verificação acontece no servidor; ao cliente só
/// retornam mensagens genéricas.
#[server]
pub async fn login(email: String, senha: String) -> Result<(), ServerFnError> {
    use crate::error::AppError;
    use tower_sessions::Session;

    let pool = expect_context::<sqlx::PgPool>();
    let session: Session = leptos_axum::extract()
        .await
        .map_err(|_| ServerFnError::new("Sessão indisponível."))?;

    match crate::server::auth::autenticar(&pool, &email, &senha).await {
        Ok(u) => {
            session
                .insert(SESSION_UID, u.id)
                .await
                .map_err(|_| ServerFnError::new("Erro ao iniciar a sessão."))?;
            crate::server::auth::auditar(&pool, Some(u.id), "auth.login").await;
            Ok(())
        }
        Err(AppError::RateLimited) => Err(ServerFnError::new(
            "Muitas tentativas. Tente novamente em alguns minutos.",
        )),
        Err(_) => Err(ServerFnError::new("Credenciais inválidas.")),
    }
}

/// Encerra a sessão atual.
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use tower_sessions::Session;

    let session: Session = leptos_axum::extract()
        .await
        .map_err(|_| ServerFnError::new("Sessão indisponível."))?;

    if let Ok(Some(uid)) = session.get::<uuid::Uuid>(SESSION_UID).await {
        let pool = expect_context::<sqlx::PgPool>();
        crate::server::auth::auditar(&pool, Some(uid), "auth.logout").await;
    }
    session
        .flush()
        .await
        .map_err(|_| ServerFnError::new("Erro ao encerrar a sessão."))?;
    Ok(())
}

/// Retorna o usuário logado (ou `None`), revalidando no banco a cada chamada.
#[server]
pub async fn usuario_atual() -> Result<Option<UsuarioSessao>, ServerFnError> {
    use tower_sessions::Session;

    let session: Session = leptos_axum::extract()
        .await
        .map_err(|_| ServerFnError::new("Sessão indisponível."))?;

    let Some(uid) = session
        .get::<uuid::Uuid>(SESSION_UID)
        .await
        .map_err(|_| ServerFnError::new("Erro de sessão."))?
    else {
        return Ok(None);
    };

    let pool = expect_context::<sqlx::PgPool>();
    crate::server::auth::carregar_sessao(&pool, uid)
        .await
        .map_err(|_| ServerFnError::new("Erro interno."))
}

/// Guarda de RBAC para server functions protegidas (usada a partir da Fase 7):
/// garante usuário autenticado e papel mínimo. Server-only.
#[cfg(feature = "ssr")]
pub async fn exigir_papel(
    minimo: crate::server::rbac::Papel,
) -> Result<UsuarioSessao, ServerFnError> {
    use crate::server::rbac::Papel;

    let u = usuario_atual()
        .await?
        .ok_or_else(|| ServerFnError::new("Não autenticado."))?;
    let papel = Papel::from_db(&u.papel).ok_or_else(|| ServerFnError::new("Acesso negado."))?;
    if !papel.atende(minimo) {
        return Err(ServerFnError::new("Acesso negado."));
    }
    Ok(u)
}
