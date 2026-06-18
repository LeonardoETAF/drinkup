use leptos::prelude::*;

/// Solicita a redefinição de senha. Resposta SEMPRE genérica (não revela se o
/// e-mail existe). Se houver usuário ativo, gera o token e envia o link.
#[server]
pub async fn solicitar_redefinicao(email: String) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();

    if let Ok(Some(pedido)) = crate::server::recuperacao::solicitar(&pool, &email).await {
        let link = format!(
            "{}/admin/redefinir-senha?token={}",
            crate::components::SITE_URL,
            pedido.token
        );
        crate::server::email::enviar_link_redefinicao(&pedido.email, &pedido.nome, &link).await;
    }
    Ok(())
}

/// Define uma nova senha a partir do token recebido por e-mail.
#[server]
pub async fn redefinir_senha(token: String, nova_senha: String) -> Result<(), ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    match crate::server::recuperacao::redefinir(&pool, &token, &nova_senha).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => Err(ServerFnError::new(
            "A senha deve ter ao menos 8 caracteres.",
        )),
        Err(_) => Err(ServerFnError::new(
            "Link inválido ou expirado. Solicite um novo.",
        )),
    }
}
