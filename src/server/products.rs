//! Repositório de catálogo (produtos/categorias). Queries verificadas em
//! compilação pelo SQLx. Server-only — nunca vai ao cliente.
use sqlx::PgPool;

use crate::domain::{
    Categoria, FiltroProdutos, PaginaProdutos, ProdutoDetalhe, ProdutoImagem, ProdutoResumo,
};

const POR_PAGINA_PADRAO: i64 = 12;
const POR_PAGINA_MAX: i64 = 60;

/// Resolve (por_pagina, offset) a partir do filtro, com limites seguros.
fn limites(filtro: &FiltroProdutos) -> (i64, i64) {
    let por_pagina = match filtro.por_pagina {
        0 => POR_PAGINA_PADRAO,
        n => (i64::from(n)).min(POR_PAGINA_MAX),
    };
    let pagina = i64::from(filtro.pagina.max(1));
    let offset = (pagina - 1) * por_pagina;
    (por_pagina, offset)
}

/// Lista produtos ativos com filtros opcionais e paginação.
pub async fn listar(pool: &PgPool, filtro: &FiltroProdutos) -> Result<PaginaProdutos, sqlx::Error> {
    let (por_pagina, offset) = limites(filtro);

    let itens = sqlx::query_as!(
        ProdutoResumo,
        r#"
        SELECT
            p.id            AS "id!",
            p.nome          AS "nome!",
            p.slug          AS "slug!",
            p.capacidade_ml,
            p.material,
            p.cor,
            p.destaque      AS "destaque!",
            (SELECT pi.url FROM produto_imagens pi
               WHERE pi.produto_id = p.id AND pi.principal LIMIT 1) AS "imagem_url?"
        FROM produtos p
        LEFT JOIN categorias c ON c.id = p.categoria_id
        LEFT JOIN categorias sc ON sc.id = p.subcategoria_id
        WHERE p.ativo
          AND ($1::text IS NULL OR c.slug = $1)
          AND ($2::text IS NULL OR sc.slug = $2)
          AND ($3::text IS NULL OR p.material = $3)
          AND ($4::text IS NULL OR p.cor = $4)
          AND ($5::text IS NULL OR lower(p.nome) LIKE '%' || lower($5) || '%')
        ORDER BY p.destaque DESC, p.nome
        LIMIT $6 OFFSET $7
        "#,
        filtro.categoria_slug.as_deref(),
        filtro.subcategoria_slug.as_deref(),
        filtro.material.as_deref(),
        filtro.cor.as_deref(),
        filtro.busca.as_deref(),
        por_pagina,
        offset,
    )
    .fetch_all(pool)
    .await?;

    let total = sqlx::query_scalar!(
        r#"
        SELECT count(*) AS "total!"
        FROM produtos p
        LEFT JOIN categorias c ON c.id = p.categoria_id
        LEFT JOIN categorias sc ON sc.id = p.subcategoria_id
        WHERE p.ativo
          AND ($1::text IS NULL OR c.slug = $1)
          AND ($2::text IS NULL OR sc.slug = $2)
          AND ($3::text IS NULL OR p.material = $3)
          AND ($4::text IS NULL OR p.cor = $4)
          AND ($5::text IS NULL OR lower(p.nome) LIKE '%' || lower($5) || '%')
        "#,
        filtro.categoria_slug.as_deref(),
        filtro.subcategoria_slug.as_deref(),
        filtro.material.as_deref(),
        filtro.cor.as_deref(),
        filtro.busca.as_deref(),
    )
    .fetch_one(pool)
    .await?;

    Ok(PaginaProdutos {
        itens,
        total,
        pagina: u32::try_from(offset / por_pagina + 1).unwrap_or(1),
        por_pagina: u32::try_from(por_pagina).unwrap_or(0),
    })
}

/// Busca um produto ativo pelo slug (com imagens e nome da categoria).
pub async fn por_slug(pool: &PgPool, slug: &str) -> Result<Option<ProdutoDetalhe>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT
            p.id AS "id!", p.nome AS "nome!", p.slug AS "slug!",
            p.descricao, p.capacidade_ml, p.material, p.cor,
            p.altura_mm, p.diametro_mm, p.personalizavel AS "personalizavel!",
            c.nome AS "categoria_nome?", sc.nome AS "subcategoria_nome?"
        FROM produtos p
        LEFT JOIN categorias c ON c.id = p.categoria_id
        LEFT JOIN categorias sc ON sc.id = p.subcategoria_id
        WHERE p.slug = $1 AND p.ativo
        "#,
        slug
    )
    .fetch_optional(pool)
    .await?;

    let Some(r) = row else {
        return Ok(None);
    };

    let imagens = sqlx::query_as!(
        ProdutoImagem,
        r#"
        SELECT url AS "url!", alt
        FROM produto_imagens
        WHERE produto_id = $1
        ORDER BY principal DESC, ordem
        "#,
        r.id
    )
    .fetch_all(pool)
    .await?;

    Ok(Some(ProdutoDetalhe {
        id: r.id,
        nome: r.nome,
        slug: r.slug,
        descricao: r.descricao,
        capacidade_ml: r.capacidade_ml,
        material: r.material,
        cor: r.cor,
        altura_mm: r.altura_mm,
        diametro_mm: r.diametro_mm,
        personalizavel: r.personalizavel,
        categoria_nome: r.categoria_nome,
        subcategoria_nome: r.subcategoria_nome,
        imagens,
    }))
}

/// Lista categorias e subcategorias ativas (para filtros/menus). O `parent_id`
/// permite agrupar subcategorias sob suas categorias na vitrine.
pub async fn listar_categorias(pool: &PgPool) -> Result<Vec<Categoria>, sqlx::Error> {
    sqlx::query_as!(
        Categoria,
        r#"
        SELECT id AS "id!", nome AS "nome!", slug AS "slug!", descricao, parent_id
        FROM categorias
        WHERE ativo
        ORDER BY ordem, nome
        "#
    )
    .fetch_all(pool)
    .await
}
