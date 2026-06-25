//! Inscritos em "Novidades" (newsletter por WhatsApp). Server-only.
//! Validação real no servidor (frontend burro); telefone único (dedupe).
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{InscritoResumo, PaginaInscritos};
use crate::error::AppError;

/// Classificações válidas de um inscrito (espelham o CHECK da tabela).
pub const CLASSIFICACOES: [&str; 4] = ["novo", "cliente", "potencial", "inativo"];

fn interno(e: sqlx::Error, ctx: &str) -> AppError {
    tracing::error!(error = %e, "{ctx}");
    AppError::Internal
}

/// Inscreve um WhatsApp (11 dígitos). Idempotente: repetido é ignorado.
pub async fn inscrever(pool: &PgPool, telefone: &str) -> Result<(), AppError> {
    if !crate::domain::whatsapp_valido(telefone) {
        return Err(AppError::Validation);
    }
    let digitos: String = telefone.chars().filter(|c| c.is_ascii_digit()).collect();
    let formatado = crate::domain::mascara_telefone(&digitos);
    sqlx::query!(
        "INSERT INTO novidades_inscritos (telefone) VALUES ($1) ON CONFLICT (telefone) DO NOTHING",
        formatado,
    )
    .execute(pool)
    .await
    .map_err(|e| interno(e, "falha ao inscrever em novidades"))?;
    Ok(())
}

/// Lista inscritos (busca por WhatsApp). Limite de 200.
pub async fn listar(pool: &PgPool, busca: Option<&str>) -> Result<PaginaInscritos, sqlx::Error> {
    let itens = sqlx::query_as!(
        InscritoResumo,
        r#"
        SELECT id AS "id!", telefone AS "telefone!", classificacao AS "classificacao!",
               to_char(created_at, 'DD/MM/YYYY') AS "inscricao!"
        FROM novidades_inscritos
        WHERE ($1::text IS NULL OR telefone ILIKE '%' || $1 || '%')
        ORDER BY created_at DESC
        LIMIT 200
        "#,
        busca,
    )
    .fetch_all(pool)
    .await?;

    let total = sqlx::query_scalar!(
        r#"SELECT count(*) AS "c!" FROM novidades_inscritos
           WHERE ($1::text IS NULL OR telefone ILIKE '%' || $1 || '%')"#,
        busca,
    )
    .fetch_one(pool)
    .await?;

    Ok(PaginaInscritos { itens, total })
}

/// Atualiza a classificação de um inscrito (valida o valor no servidor).
pub async fn atualizar_classificacao(
    pool: &PgPool,
    id: Uuid,
    classificacao: &str,
) -> Result<(), AppError> {
    if !CLASSIFICACOES.contains(&classificacao) {
        return Err(AppError::Validation);
    }
    sqlx::query!(
        "UPDATE novidades_inscritos SET classificacao = $2 WHERE id = $1",
        id,
        classificacao,
    )
    .execute(pool)
    .await
    .map_err(|e| interno(e, "falha ao atualizar classificação"))?;
    Ok(())
}

/// Remove um inscrito.
pub async fn excluir(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM novidades_inscritos WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(|e| interno(e, "falha ao excluir inscrito"))?;
    Ok(())
}
