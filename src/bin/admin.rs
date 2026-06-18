//! Ferramenta para criar/atualizar um usuário do painel (define a senha real).
//! Uso:
//!   cargo run --bin admin --no-default-features --features ssr -- \
//!       <email> <nome> <senha> [papel: admin|gerente|editor]
#![forbid(unsafe_code)]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use drinkup::server::{auth, db};

    let _ = dotenvy::dotenv();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("uso: admin <email> <nome> <senha> [papel: admin|gerente|editor]");
        std::process::exit(2);
    }

    let email = args[1].trim().to_lowercase();
    let nome = args[2].clone();
    let senha = &args[3];
    let papel = args
        .get(4)
        .map(String::as_str)
        .unwrap_or("admin")
        .to_string();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL não definida");
    let pool = db::create_pool(&url)
        .await
        .expect("falha ao conectar no banco");
    let hash = auth::gerar_hash(senha).expect("falha ao gerar hash da senha");

    sqlx::query!(
        r#"INSERT INTO usuarios (nome, email, senha_hash, papel)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (lower(email))
           DO UPDATE SET senha_hash = EXCLUDED.senha_hash,
                         nome       = EXCLUDED.nome,
                         papel      = EXCLUDED.papel,
                         ativo      = true"#,
        nome,
        email,
        hash,
        papel
    )
    .execute(&pool)
    .await
    .expect("falha ao salvar o usuário");

    println!("usuário '{email}' criado/atualizado (papel: {papel})");
}

#[cfg(not(feature = "ssr"))]
fn main() {}
