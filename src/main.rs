//! Entrada do servidor DRINKUP (Axum + Leptos SSR).
#![forbid(unsafe_code)]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::extract::DefaultBodyLimit;
    use axum::routing::post;
    use axum::Router;
    use drinkup::app::{shell, App};
    use drinkup::server::{db, uploads};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use time::Duration;
    use tower_http::services::ServeDir;
    use tower_sessions::cookie::SameSite;
    use tower_sessions::{Expiry, SessionManagerLayer};
    use tower_sessions_sqlx_store::PostgresStore;
    use tracing_subscriber::EnvFilter;

    // Em desenvolvimento, carrega variáveis de um arquivo .env (se existir).
    let _ = dotenvy::dotenv();

    // Logs estruturados, controlados por RUST_LOG.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Banco: conecta e aplica migrations pendentes antes de servir.
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL não definida");
    let pool = db::create_pool(&database_url)
        .await
        .expect("falha ao conectar no PostgreSQL");
    db::run_migrations(&pool)
        .await
        .expect("falha ao aplicar migrations");
    log!("PostgreSQL conectado e migrations aplicadas");

    // Sessões server-side em Postgres (a store cria/migra sua própria tabela).
    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("falha ao migrar a tabela de sessões");
    let session_layer = SessionManagerLayer::new(session_store)
        .with_http_only(true)
        .with_same_site(SameSite::Lax)
        .with_secure(!cfg!(debug_assertions)) // Secure em release (HTTPS), liberado em dev
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    // Garante o diretório de uploads (servido em /uploads).
    let _ = tokio::fs::create_dir_all(uploads::DIR_UPLOADS).await;

    let conf = get_configuration(None).expect("falha ao ler a configuração do Leptos");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/upload-imagem", post(upload_handler))
        .route(
            "/sitemap.xml",
            axum::routing::get({
                let pool = pool.clone();
                move || sitemap_handler(pool.clone())
            }),
        )
        .nest_service("/uploads", ServeDir::new(uploads::DIR_UPLOADS))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                // Disponibiliza o pool às server functions via contexto.
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(DefaultBodyLimit::max(6 * 1024 * 1024))
        .layer(axum::middleware::from_fn(guarda_admin))
        .layer(session_layer)
        .layer(axum::middleware::from_fn(verifica_origem))
        .layer(axum::middleware::from_fn(cabecalhos_seguranca))
        .with_state(leptos_options);

    log!("DRINKUP ouvindo em http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("falha ao abrir o socket TCP");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("falha ao iniciar o servidor");
}

/// Adiciona cabeçalhos de segurança a todas as respostas. HSTS só em release
/// (produção/HTTPS). A CSP é definida no render (com nonce), não aqui.
#[cfg(feature = "ssr")]
async fn cabecalhos_seguranca(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::http::{header, HeaderName, HeaderValue};

    // Caminho capturado antes de `req` ser consumido por `next`.
    let path = req.uri().path().to_string();
    let mut resp = next.run(req).await;
    let h = resp.headers_mut();
    // Imagens enviadas têm nome aleatório/estável → cache longo e imutável.
    if path.starts_with("/uploads/") {
        h.insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        );
    }
    h.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    h.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    h.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    h.insert(
        HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=(), browsing-topics=()"),
    );
    h.insert(
        HeaderName::from_static("cross-origin-opener-policy"),
        HeaderValue::from_static("same-origin"),
    );
    if !cfg!(debug_assertions) {
        h.insert(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=63072000; includeSubDomains"),
        );
    }
    resp
}

/// Mitiga CSRF (defesa em profundidade, além do cookie `SameSite=Lax`): em
/// requisições mutáveis para `/api/*` e `/upload-imagem`, se houver cabeçalho
/// `Origin`, ele precisa casar com o `Host`. Sem `Origin` (ex.: clientes não-
/// navegador) segue normalmente.
#[cfg(feature = "ssr")]
async fn verifica_origem(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::http::{header, Method, StatusCode};
    use axum::response::IntoResponse;

    let mutavel = matches!(
        *req.method(),
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    );
    let path = req.uri().path();
    let alvo = path.starts_with("/api/") || path == "/upload-imagem";

    if mutavel && alvo {
        if let Some(origin) = req
            .headers()
            .get(header::ORIGIN)
            .and_then(|v| v.to_str().ok())
        {
            let host = req
                .headers()
                .get(header::HOST)
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default();
            let autoridade = origin.split_once("://").map(|(_, a)| a).unwrap_or_default();
            if host.is_empty() || autoridade != host {
                tracing::warn!(%origin, %host, "origem rejeitada (possível CSRF)");
                return (StatusCode::FORBIDDEN, "Origem inválida.").into_response();
            }
        }
    }
    next.run(req).await
}

/// Serve o `sitemap.xml` gerado a partir das páginas públicas + produtos ativos.
#[cfg(feature = "ssr")]
async fn sitemap_handler(pool: sqlx::PgPool) -> axum::response::Response {
    use axum::http::{header, StatusCode};
    use axum::response::IntoResponse;

    match drinkup::server::sitemap::gerar_xml(&pool, drinkup::components::SITE_URL).await {
        Ok(xml) => (
            [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
            xml,
        )
            .into_response(),
        Err(e) => {
            tracing::error!(error = %e, "falha ao gerar sitemap");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Guarda de rota server-side: barra acesso não autenticado a `/admin/*`
/// (exceto `/admin/login`) com um 302, antes de o Leptos renderizar.
/// Defesa em profundidade — a autorização real das ações fica nas server functions.
#[cfg(feature = "ssr")]
async fn guarda_admin(
    session: tower_sessions::Session,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::{IntoResponse, Redirect};

    let path = req.uri().path();
    let publico = path == "/admin/login"
        || path == "/admin/recuperar-senha"
        || path == "/admin/redefinir-senha";
    let protegido = (path == "/admin" || path.starts_with("/admin/")) && !publico;
    if protegido {
        let autenticado = session
            .get::<uuid::Uuid>("uid")
            .await
            .unwrap_or(None)
            .is_some();
        if !autenticado {
            return Redirect::to("/admin/login").into_response();
        }
    }
    next.run(req).await
}

/// Recebe upload de imagem (multipart). Exige login; valida tipo/tamanho no
/// servidor e grava num caminho controlado. Responde com a URL pública (texto).
#[cfg(feature = "ssr")]
async fn upload_handler(
    session: tower_sessions::Session,
    mut multipart: axum::extract::Multipart,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let autenticado = session
        .get::<uuid::Uuid>("uid")
        .await
        .unwrap_or(None)
        .is_some();
    if !autenticado {
        return (StatusCode::UNAUTHORIZED, "Não autenticado.").into_response();
    }

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() != Some("imagem") {
            continue;
        }
        let Ok(bytes) = field.bytes().await else {
            return (StatusCode::BAD_REQUEST, "Falha ao ler o arquivo.").into_response();
        };
        return match drinkup::server::uploads::salvar_imagem(&bytes).await {
            Ok(url) => (StatusCode::OK, url).into_response(),
            Err(_) => (
                StatusCode::BAD_REQUEST,
                "Imagem inválida (use JPG, PNG ou WEBP até 5MB).",
            )
                .into_response(),
        };
    }
    (StatusCode::BAD_REQUEST, "Nenhum arquivo enviado.").into_response()
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // Sem main no cliente — a hidratação ocorre em `src/hydration.rs`.
}
