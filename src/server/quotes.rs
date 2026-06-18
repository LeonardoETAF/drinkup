//! Persistência de orçamentos/leads. A validação real acontece aqui (frontend
//! burro): nunca confiamos nos dados vindos do cliente.
use sqlx::PgPool;

use crate::domain::NovoOrcamento;
use crate::error::AppError;

const MAX_NOME: usize = 120;
const MAX_CONTATO: usize = 60;
const MAX_EMAIL: usize = 160;
const MAX_MENSAGEM: usize = 2000;
const MAX_ITENS: usize = 50;

/// Valida e persiste um pedido de orçamento (lead + itens) numa transação.
pub async fn criar(pool: &PgPool, dados: &NovoOrcamento) -> Result<(), AppError> {
    let nome = dados.nome.trim();
    let contato = dados.contato.trim();

    if nome.is_empty() || nome.chars().count() > MAX_NOME {
        return Err(AppError::Validation);
    }
    if contato.is_empty() || contato.chars().count() > MAX_CONTATO {
        return Err(AppError::Validation);
    }
    if let Some(email) = dados.email.as_deref() {
        let email = email.trim();
        if !email.is_empty() && (email.chars().count() > MAX_EMAIL || !email.contains('@')) {
            return Err(AppError::Validation);
        }
    }
    if dados
        .mensagem
        .as_deref()
        .is_some_and(|m| m.chars().count() > MAX_MENSAGEM)
    {
        return Err(AppError::Validation);
    }
    if dados.itens.len() > MAX_ITENS {
        return Err(AppError::Validation);
    }

    // Throttle simples: mesmo contato não pode reenviar em menos de 30s.
    let recente = sqlx::query_scalar!(
        r#"SELECT EXISTS(
            SELECT 1 FROM leads WHERE contato = $1 AND created_at > now() - interval '30 seconds'
        ) AS "recente!""#,
        contato
    )
    .fetch_one(pool)
    .await
    .map_err(interno)?;
    if recente {
        return Err(AppError::RateLimited);
    }

    let mut tx = pool.begin().await.map_err(interno)?;

    let lead_id = sqlx::query_scalar!(
        r#"
        INSERT INTO leads (nome, contato, email, mensagem, origem)
        VALUES ($1, $2, $3, $4, 'site')
        RETURNING id
        "#,
        nome,
        contato,
        dados.email.as_deref(),
        dados.mensagem.as_deref(),
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(interno)?;

    for item in &dados.itens {
        sqlx::query!(
            r#"
            INSERT INTO lead_itens (lead_id, produto_id, descricao, quantidade)
            VALUES ($1, $2, $3, $4)
            "#,
            lead_id,
            item.produto_id,
            item.descricao.as_deref(),
            item.quantidade.max(1),
        )
        .execute(&mut *tx)
        .await
        .map_err(interno)?;
    }

    tx.commit().await.map_err(interno)?;
    Ok(())
}

/// Loga o erro real no servidor e devolve um erro genérico ao chamador.
fn interno(e: sqlx::Error) -> AppError {
    tracing::error!(error = %e, "erro de banco ao registrar orçamento");
    AppError::Internal
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Integração (requer Postgres em DATABASE_URL): valida persistência,
    /// throttle e validação. Pula silenciosamente se não houver banco.
    #[tokio::test]
    async fn valida_persiste_e_aplica_throttle() {
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
        let contato = format!("teste-{n}");
        let dados = NovoOrcamento {
            nome: "Fulano Teste".to_string(),
            contato: contato.clone(),
            email: None,
            mensagem: Some("Mensagem de teste".to_string()),
            itens: vec![],
        };

        // Primeiro envio: válido e persistido.
        assert!(criar(&pool, &dados).await.is_ok());
        // Reenvio imediato do mesmo contato: bloqueado pelo throttle.
        assert_eq!(criar(&pool, &dados).await, Err(AppError::RateLimited));
        // Nome vazio: validação falha.
        let invalido = NovoOrcamento {
            nome: "  ".to_string(),
            ..dados.clone()
        };
        assert_eq!(criar(&pool, &invalido).await, Err(AppError::Validation));

        // Limpeza.
        sqlx::query!("DELETE FROM leads WHERE contato = $1", contato)
            .execute(&pool)
            .await
            .unwrap();
    }
}
