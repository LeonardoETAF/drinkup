//! Redefinição de senha por token (uso único, com expiração). Server-only.
//!
//! O token é aleatório e de alta entropia; no banco guardamos apenas seu hash
//! SHA-256. A resposta ao cliente é sempre genérica (sem revelar se o e-mail
//! existe). A troca de senha é transacional e invalida o token.
use argon2::password_hash::rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::error::AppError;
use crate::server::auth::gerar_hash;

const VALIDADE_HORAS: i32 = 1;
use crate::server::auth::SENHA_MIN;

/// Dados necessários para enviar o e-mail (token em claro, só em memória).
pub struct PedidoReset {
    pub email: String,
    pub nome: String,
    pub token: String,
}

fn gerar_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn hash_token(token: &str) -> String {
    let mut h = Sha256::new();
    h.update(token.as_bytes());
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Cria um token se o e-mail pertencer a um usuário ativo. Invalida pedidos
/// anteriores não usados. Retorna `None` (silenciosamente) se não houver usuário.
pub async fn solicitar(pool: &PgPool, email: &str) -> Result<Option<PedidoReset>, AppError> {
    let email = email.trim().to_lowercase();
    let usuario = sqlx::query!(
        r#"SELECT id, nome, email FROM usuarios WHERE lower(email) = $1 AND ativo = true"#,
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(banco)?;

    let Some(u) = usuario else { return Ok(None) };

    sqlx::query!(
        "UPDATE password_resets SET usado_em = now() WHERE usuario_id = $1 AND usado_em IS NULL",
        u.id
    )
    .execute(pool)
    .await
    .map_err(banco)?;

    let token = gerar_token();
    sqlx::query!(
        r#"INSERT INTO password_resets (usuario_id, token_hash, expira_em)
           VALUES ($1, $2, now() + make_interval(hours => $3))"#,
        u.id,
        hash_token(&token),
        VALIDADE_HORAS,
    )
    .execute(pool)
    .await
    .map_err(banco)?;

    Ok(Some(PedidoReset {
        email: u.email,
        nome: u.nome,
        token,
    }))
}

/// Redefine a senha a partir de um token válido (não usado e não expirado).
pub async fn redefinir(pool: &PgPool, token: &str, nova_senha: &str) -> Result<(), AppError> {
    if nova_senha.chars().count() < SENHA_MIN {
        return Err(AppError::Validation);
    }

    let registro = sqlx::query!(
        r#"SELECT id, usuario_id FROM password_resets
           WHERE token_hash = $1 AND usado_em IS NULL AND expira_em > now()"#,
        hash_token(token.trim())
    )
    .fetch_optional(pool)
    .await
    .map_err(banco)?;

    let Some(r) = registro else {
        return Err(AppError::Unauthorized);
    };

    let hash = gerar_hash(nova_senha)?;
    let mut tx = pool.begin().await.map_err(banco)?;
    sqlx::query!(
        "UPDATE usuarios SET senha_hash = $2 WHERE id = $1",
        r.usuario_id,
        hash
    )
    .execute(&mut *tx)
    .await
    .map_err(banco)?;
    sqlx::query!(
        "UPDATE password_resets SET usado_em = now() WHERE id = $1",
        r.id
    )
    .execute(&mut *tx)
    .await
    .map_err(banco)?;
    tx.commit().await.map_err(banco)?;
    Ok(())
}

fn banco(e: sqlx::Error) -> AppError {
    tracing::error!(error = %e, "erro de banco em recuperacao");
    AppError::Internal
}
