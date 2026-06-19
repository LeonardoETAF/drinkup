//! Configurações da loja (chave/valor). Server-only.
use std::collections::HashMap;

use sqlx::PgPool;

use crate::domain::Configuracoes;
use crate::error::AppError;

/// Lê todas as configurações, preenchendo o DTO (chaves ausentes => vazio).
pub async fn obter(pool: &PgPool) -> Result<Configuracoes, sqlx::Error> {
    let rows = sqlx::query!("SELECT chave, valor FROM configuracoes")
        .fetch_all(pool)
        .await?;
    let mapa: HashMap<String, String> = rows.into_iter().map(|r| (r.chave, r.valor)).collect();
    let get = |k: &str| mapa.get(k).cloned().unwrap_or_default();

    Ok(Configuracoes {
        nome_loja: get("nome_loja"),
        cnpj: get("cnpj"),
        telefone: get("telefone"),
        endereco: get("endereco"),
        horario_semana: get("horario_semana"),
        horario_sabado: get("horario_sabado"),
        horario_domingo: get("horario_domingo"),
        facebook: get("social_facebook"),
        instagram: get("social_instagram"),
        // Ausente => ativo (preserva o comportamento anterior).
        facebook_ativo: get("social_facebook_ativo") != "false",
        instagram_ativo: get("social_instagram_ativo") != "false",
        whatsapp_ativo: get("social_whatsapp_ativo") != "false",
    })
}

/// Grava (upsert) todas as configurações.
pub async fn salvar(pool: &PgPool, c: &Configuracoes) -> Result<(), AppError> {
    let pares = [
        ("nome_loja", c.nome_loja.trim()),
        ("cnpj", c.cnpj.trim()),
        ("telefone", c.telefone.trim()),
        ("endereco", c.endereco.trim()),
        ("horario_semana", c.horario_semana.trim()),
        ("horario_sabado", c.horario_sabado.trim()),
        ("horario_domingo", c.horario_domingo.trim()),
        ("social_facebook", c.facebook.trim()),
        ("social_instagram", c.instagram.trim()),
        ("social_facebook_ativo", if c.facebook_ativo { "true" } else { "false" }),
        ("social_instagram_ativo", if c.instagram_ativo { "true" } else { "false" }),
        ("social_whatsapp_ativo", if c.whatsapp_ativo { "true" } else { "false" }),
    ];
    for (chave, valor) in pares {
        if valor.chars().count() > 300 {
            return Err(AppError::Validation);
        }
        sqlx::query!(
            r#"INSERT INTO configuracoes (chave, valor) VALUES ($1, $2)
               ON CONFLICT (chave) DO UPDATE SET valor = EXCLUDED.valor"#,
            chave,
            valor,
        )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "erro ao salvar configurações");
            AppError::Internal
        })?;
    }
    Ok(())
}
