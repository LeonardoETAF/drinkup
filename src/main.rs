//! Entrada do servidor DRINKUP (Axum + Leptos SSR).
#![forbid(unsafe_code)]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use drinkup::app::{shell, App};
    use drinkup::server::db;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use time::Duration;
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

    let conf = get_configuration(None).expect("falha ao ler a configuração do Leptos");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
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
        .layer(axum::middleware::from_fn(guarda_admin))
        .layer(session_layer)
        .with_state(leptos_options);

    log!("DRINKUP ouvindo em http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("falha ao abrir o socket TCP");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("falha ao iniciar o servidor");
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
    let protegido = (path == "/admin" || path.starts_with("/admin/")) && path != "/admin/login";
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

#[cfg(not(feature = "ssr"))]
fn main() {
    // Sem main no cliente — a hidratação ocorre em `src/hydration.rs`.
}
