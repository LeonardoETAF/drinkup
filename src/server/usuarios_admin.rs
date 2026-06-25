//! CRUD de usuários do painel (somente admin). Server-only.
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{UsuarioForm, UsuarioLista};
use crate::error::AppError;
use crate::server::auth::gerar_hash;

const PAPEIS: [&str; 4] = ["admin", "gerente", "editor", "visualizador"];

/// Lista os usuários do painel.
pub async fn listar(pool: &PgPool) -> Result<Vec<UsuarioLista>, sqlx::Error> {
    sqlx::query_as!(
        UsuarioLista,
        r#"
        SELECT id AS "id!", nome AS "nome!", email AS "email!", papel AS "papel!",
               ativo AS "ativo!",
               to_char(ultimo_login, 'DD/MM/YYYY HH24:MI') AS "ultimo_login?"
        FROM usuarios
        ORDER BY nome
        "#
    )
    .fetch_all(pool)
    .await
}

/// Carrega um usuário para edição (sem expor a senha).
pub async fn obter_form(pool: &PgPool, id: Uuid) -> Result<Option<UsuarioForm>, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT id, nome, email, papel, ativo, menus FROM usuarios WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| UsuarioForm {
        id: Some(r.id),
        nome: r.nome,
        email: r.email,
        papel: r.papel,
        ativo: r.ativo,
        senha: None,
        menus: r.menus,
    }))
}

/// Cria ou atualiza um usuário. Na criação a senha é obrigatória; na edição,
/// senha vazia mantém a atual.
pub async fn salvar(pool: &PgPool, form: &UsuarioForm) -> Result<Uuid, AppError> {
    let nome = form.nome.trim();
    let email = form.email.trim().to_lowercase();
    if nome.is_empty() || nome.chars().count() > 120 {
        return Err(AppError::Validation);
    }
    if email.is_empty() || !email.contains('@') || email.chars().count() > 160 {
        return Err(AppError::Validation);
    }
    if !PAPEIS.contains(&form.papel.as_str()) {
        return Err(AppError::Validation);
    }
    let senha = form
        .senha
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    // Quando há senha (criação ou troca), exige o comprimento mínimo.
    if senha.is_some_and(|s| s.chars().count() < crate::server::auth::SENHA_MIN) {
        return Err(AppError::Validation);
    }

    // Só permite chaves de menu conhecidas.
    let menus: Vec<String> = form
        .menus
        .iter()
        .filter(|m| crate::server::rbac::MENUS.contains(&m.as_str()))
        .cloned()
        .collect();

    match form.id {
        Some(id) => {
            sqlx::query!(
                "UPDATE usuarios SET nome = $2, email = $3, papel = $4, ativo = $5, menus = $6 \
                 WHERE id = $1",
                id,
                nome,
                email,
                form.papel,
                form.ativo,
                &menus,
            )
            .execute(pool)
            .await
            .map_err(banco)?;
            if let Some(senha) = senha {
                let hash = gerar_hash(senha)?;
                sqlx::query!(
                    "UPDATE usuarios SET senha_hash = $2 WHERE id = $1",
                    id,
                    hash
                )
                .execute(pool)
                .await
                .map_err(banco)?;
            }
            Ok(id)
        }
        None => {
            let senha = senha.ok_or(AppError::Validation)?;
            let hash = gerar_hash(senha)?;
            sqlx::query_scalar!(
                r#"INSERT INTO usuarios (nome, email, senha_hash, papel, ativo, menus)
                   VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"#,
                nome,
                email,
                hash,
                form.papel,
                form.ativo,
                &menus,
            )
            .fetch_one(pool)
            .await
            .map_err(|e| {
                // Violação de unicidade do e-mail.
                if let sqlx::Error::Database(db) = &e {
                    if db.is_unique_violation() {
                        return AppError::Validation;
                    }
                }
                banco(e)
            })
        }
    }
}

/// Exclui um usuário (impede excluir a si mesmo).
pub async fn excluir(pool: &PgPool, id: Uuid, atual: Uuid) -> Result<(), AppError> {
    if id == atual {
        return Err(AppError::Validation);
    }
    sqlx::query!("DELETE FROM usuarios WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(banco)?;
    Ok(())
}

fn banco(e: sqlx::Error) -> AppError {
    tracing::error!(error = %e, "erro de banco em usuarios_admin");
    AppError::Internal
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Integração (requer Postgres em DATABASE_URL): senha < 8 é rejeitada;
    /// senha >= 8 cria o usuário.
    #[tokio::test]
    async fn senha_minima_obrigatoria() {
        let Ok(url) = std::env::var("DATABASE_URL") else {
            return;
        };
        let Ok(pool) = crate::server::db::create_pool(&url).await else {
            return;
        };
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let base = UsuarioForm {
            id: None,
            nome: "Teste".into(),
            email: format!("seg{n}@test.com"),
            papel: "editor".into(),
            ativo: true,
            senha: Some("1234567".into()), // 7 < 8
            menus: vec![],
        };
        assert!(matches!(
            salvar(&pool, &base).await,
            Err(AppError::Validation)
        ));

        let ok = UsuarioForm {
            senha: Some("12345678".into()),
            ..base
        };
        let id = salvar(&pool, &ok).await.expect("senha de 8 deve criar");
        let _ = sqlx::query!("DELETE FROM usuarios WHERE id = $1", id)
            .execute(&pool)
            .await;
    }
}
