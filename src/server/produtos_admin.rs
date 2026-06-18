//! CRUD de produtos para o painel. Server-only; queries verificadas em compilação.
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{ProdutoForm, ProdutoLista};
use crate::error::AppError;

const MAX_NOME: usize = 160;

/// Normaliza um caractere para o slug (transliteração básica PT-BR).
fn normaliza(c: char) -> Option<char> {
    let minuscula = c.to_ascii_lowercase();
    Some(match c {
        'á' | 'à' | 'â' | 'ã' | 'ä' | 'Á' | 'À' | 'Â' | 'Ã' | 'Ä' => 'a',
        'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => 'e',
        'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => 'i',
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' | 'Ó' | 'Ò' | 'Ô' | 'Õ' | 'Ö' => 'o',
        'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => 'u',
        'ç' | 'Ç' => 'c',
        _ if minuscula.is_ascii_alphanumeric() => minuscula,
        _ => return None,
    })
}

/// Gera um slug a partir do nome.
fn slugify(s: &str) -> String {
    let mut out = String::new();
    let mut traco = false;
    for c in s.chars() {
        match normaliza(c) {
            Some(ch) => {
                out.push(ch);
                traco = false;
            }
            None => {
                if !traco && !out.is_empty() {
                    out.push('-');
                    traco = true;
                }
            }
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "produto".to_string()
    } else {
        out
    }
}

/// Lista produtos (com nome da categoria e imagem principal) para o painel.
pub async fn listar_admin(
    pool: &PgPool,
    busca: Option<&str>,
) -> Result<Vec<ProdutoLista>, sqlx::Error> {
    sqlx::query_as!(
        ProdutoLista,
        r#"
        SELECT p.id AS "id!", p.nome AS "nome!", c.nome AS "categoria?",
               p.capacidade_ml, p.ativo AS "ativo!",
               (SELECT url FROM produto_imagens pi
                  WHERE pi.produto_id = p.id AND pi.principal LIMIT 1) AS "imagem_url?"
        FROM produtos p
        LEFT JOIN categorias c ON c.id = p.categoria_id
        WHERE ($1::text IS NULL OR p.nome ILIKE '%' || $1 || '%')
        ORDER BY p.nome
        "#,
        busca
    )
    .fetch_all(pool)
    .await
}

/// Carrega um produto para edição.
pub async fn obter_form(pool: &PgPool, id: Uuid) -> Result<Option<ProdutoForm>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, categoria_id, nome, descricao, capacidade_ml, material, cor,
               altura_mm, diametro_mm, personalizavel, destaque, ativo,
               (SELECT url FROM produto_imagens pi
                  WHERE pi.produto_id = produtos.id AND pi.principal LIMIT 1) AS "imagem_url?"
        FROM produtos WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ProdutoForm {
        id: Some(r.id),
        categoria_id: r.categoria_id,
        nome: r.nome,
        descricao: r.descricao,
        capacidade_ml: r.capacidade_ml,
        material: r.material,
        cor: r.cor,
        altura_mm: r.altura_mm,
        diametro_mm: r.diametro_mm,
        personalizavel: r.personalizavel,
        destaque: r.destaque,
        ativo: r.ativo,
        imagem_url: r.imagem_url,
    }))
}

/// Cria ou atualiza um produto (valida no servidor). Retorna o id.
pub async fn salvar(pool: &PgPool, form: &ProdutoForm) -> Result<Uuid, AppError> {
    let nome = form.nome.trim();
    if nome.is_empty() || nome.chars().count() > MAX_NOME {
        return Err(AppError::Validation);
    }

    let id = match form.id {
        Some(id) => {
            sqlx::query!(
                r#"
                UPDATE produtos SET
                    categoria_id = $2, nome = $3, descricao = $4, capacidade_ml = $5,
                    material = $6, cor = $7, altura_mm = $8, diametro_mm = $9,
                    personalizavel = $10, destaque = $11, ativo = $12
                WHERE id = $1
                "#,
                id,
                form.categoria_id,
                nome,
                form.descricao.as_deref(),
                form.capacidade_ml,
                form.material.as_deref(),
                form.cor.as_deref(),
                form.altura_mm,
                form.diametro_mm,
                form.personalizavel,
                form.destaque,
                form.ativo,
            )
            .execute(pool)
            .await
            .map_err(interno)?;
            id
        }
        None => {
            let slug = slug_unico(pool, &slugify(nome)).await?;
            sqlx::query_scalar!(
                r#"
                INSERT INTO produtos
                    (categoria_id, nome, slug, descricao, capacidade_ml, material, cor,
                     altura_mm, diametro_mm, personalizavel, destaque, ativo)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                RETURNING id
                "#,
                form.categoria_id,
                nome,
                slug,
                form.descricao.as_deref(),
                form.capacidade_ml,
                form.material.as_deref(),
                form.cor.as_deref(),
                form.altura_mm,
                form.diametro_mm,
                form.personalizavel,
                form.destaque,
                form.ativo,
            )
            .fetch_one(pool)
            .await
            .map_err(interno)?
        }
    };

    // Imagem principal (definida na Parte 2b via upload).
    if let Some(url) = form.imagem_url.as_deref() {
        sqlx::query!(
            "DELETE FROM produto_imagens WHERE produto_id = $1 AND principal",
            id
        )
        .execute(pool)
        .await
        .map_err(interno)?;
        sqlx::query!(
            "INSERT INTO produto_imagens (produto_id, url, principal) VALUES ($1, $2, true)",
            id,
            url
        )
        .execute(pool)
        .await
        .map_err(interno)?;
    }

    Ok(id)
}

/// Exclui um produto (imagens removidas em cascata pela FK).
pub async fn excluir(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM produtos WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(interno)?;
    Ok(())
}

/// Garante slug único acrescentando sufixo numérico se necessário.
async fn slug_unico(pool: &PgPool, base: &str) -> Result<String, AppError> {
    let mut slug = base.to_string();
    let mut i = 1;
    while sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM produtos WHERE slug = $1) AS "e!""#,
        slug
    )
    .fetch_one(pool)
    .await
    .map_err(interno)?
    {
        slug = format!("{base}-{i}");
        i += 1;
        if i > 100 {
            return Err(AppError::Internal);
        }
    }
    Ok(slug)
}

fn interno(e: sqlx::Error) -> AppError {
    tracing::error!(error = %e, "erro de banco em produtos_admin");
    AppError::Internal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_basico() {
        assert_eq!(slugify("Taça Gin 580ml"), "taca-gin-580ml");
        assert_eq!(slugify("  Copo  Whisky!! "), "copo-whisky");
    }

    /// Integração (requer Postgres em DATABASE_URL): criar → ler → atualizar → excluir.
    #[tokio::test]
    async fn crud_produto() {
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
        let nome = format!("Produto Teste {n}");
        let form = ProdutoForm {
            nome: nome.clone(),
            capacidade_ml: Some(123),
            personalizavel: true,
            ativo: true,
            ..Default::default()
        };

        let id = salvar(&pool, &form).await.unwrap();
        let carregado = obter_form(&pool, id).await.unwrap().expect("deve existir");
        assert_eq!(carregado.nome, nome);
        assert_eq!(carregado.capacidade_ml, Some(123));

        let atualizado = ProdutoForm {
            id: Some(id),
            nome: nome.clone(),
            capacidade_ml: Some(456),
            ativo: false,
            ..Default::default()
        };
        salvar(&pool, &atualizado).await.unwrap();
        let recarregado = obter_form(&pool, id).await.unwrap().unwrap();
        assert_eq!(recarregado.capacidade_ml, Some(456));
        assert!(!recarregado.ativo);

        excluir(&pool, id).await.unwrap();
        assert!(obter_form(&pool, id).await.unwrap().is_none());
    }
}
