//! Geração do `sitemap.xml` (páginas estáticas + produtos ativos). Server-only.
use sqlx::PgPool;

/// Páginas públicas fixas do site.
const PAGINAS: [&str; 5] = ["/", "/produtos", "/contato", "/quem-somos", "/parceiros"];

/// Monta o XML do sitemap com URLs absolutas a partir de `base` (sem barra final).
pub async fn gerar_xml(pool: &PgPool, base: &str) -> Result<String, sqlx::Error> {
    let slugs =
        sqlx::query_scalar!(r#"SELECT slug FROM produtos WHERE ativo = true ORDER BY nome"#)
            .fetch_all(pool)
            .await?;

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');
    xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    xml.push('\n');

    for pagina in PAGINAS {
        empilhar(&mut xml, base, pagina);
    }
    for slug in slugs {
        empilhar(&mut xml, base, &format!("/produtos/{slug}"));
    }

    xml.push_str("</urlset>\n");
    Ok(xml)
}

fn empilhar(xml: &mut String, base: &str, caminho: &str) {
    xml.push_str("  <url><loc>");
    escapar_xml(xml, base);
    escapar_xml(xml, caminho);
    xml.push_str("</loc></url>\n");
}

/// Acrescenta `texto` ao buffer escapando as entidades XML obrigatórias.
/// Defesa em profundidade: hoje os slugs/base não contêm esses caracteres.
fn escapar_xml(xml: &mut String, texto: &str) {
    for c in texto.chars() {
        match c {
            '&' => xml.push_str("&amp;"),
            '<' => xml.push_str("&lt;"),
            '>' => xml.push_str("&gt;"),
            '"' => xml.push_str("&quot;"),
            '\'' => xml.push_str("&apos;"),
            _ => xml.push(c),
        }
    }
}
