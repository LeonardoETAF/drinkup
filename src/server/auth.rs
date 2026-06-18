//! Autenticação: hash/verificação de senha (Argon2id), login com proteção
//! contra brute force, carga da sessão e auditoria. Server-only.
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::UsuarioSessao;
use crate::error::AppError;

/// Gera um hash Argon2id (PHC string) para armazenar no banco.
pub fn gerar_hash(senha: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(senha.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| AppError::Internal)
}

/// Verifica a senha contra o hash armazenado. Falha de parsing => `false`.
fn verificar_senha(hash: &str, senha: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed) => Argon2::default()
            .verify_password(senha.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}

// --- Rate limiting em memória (anti brute force) ---
const MAX_FALHAS: u32 = 5;
const JANELA: Duration = Duration::from_secs(300);

static TENTATIVAS: LazyLock<Mutex<HashMap<String, (u32, Instant)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn trava(
    m: &Mutex<HashMap<String, (u32, Instant)>>,
) -> MutexGuard<'_, HashMap<String, (u32, Instant)>> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn bloqueado(chave: &str) -> bool {
    let mut m = trava(&TENTATIVAS);
    match m.get(chave) {
        Some((n, t)) if t.elapsed() < JANELA => *n >= MAX_FALHAS,
        Some(_) => {
            m.remove(chave);
            false
        }
        None => false,
    }
}

fn registrar_falha(chave: &str) {
    let mut m = trava(&TENTATIVAS);
    let e = m.entry(chave.to_string()).or_insert((0, Instant::now()));
    if e.1.elapsed() >= JANELA {
        *e = (0, Instant::now());
    }
    e.0 += 1;
}

fn limpar(chave: &str) {
    trava(&TENTATIVAS).remove(chave);
}

/// Autentica por e-mail + senha. Erro genérico (`Unauthorized`) tanto para
/// usuário inexistente quanto senha errada (evita enumeração).
pub async fn autenticar(
    pool: &PgPool,
    email: &str,
    senha: &str,
) -> Result<UsuarioSessao, AppError> {
    let chave = email.trim().to_lowercase();
    if bloqueado(&chave) {
        return Err(AppError::RateLimited);
    }

    let rec = sqlx::query!(
        r#"SELECT id, nome, senha_hash, papel, ativo FROM usuarios WHERE lower(email) = $1"#,
        chave
    )
    .fetch_optional(pool)
    .await
    .map_err(erro_db)?;

    let Some(u) = rec else {
        registrar_falha(&chave);
        return Err(AppError::Unauthorized);
    };
    if !u.ativo || !verificar_senha(&u.senha_hash, senha) {
        registrar_falha(&chave);
        return Err(AppError::Unauthorized);
    }

    limpar(&chave);
    let _ = sqlx::query!(
        "UPDATE usuarios SET ultimo_login = now() WHERE id = $1",
        u.id
    )
    .execute(pool)
    .await;

    Ok(UsuarioSessao {
        id: u.id,
        nome: u.nome,
        papel: u.papel,
    })
}

/// Carrega o usuário da sessão pelo id, revalidando que ainda está ativo.
pub async fn carregar_sessao(pool: &PgPool, uid: Uuid) -> Result<Option<UsuarioSessao>, AppError> {
    let rec = sqlx::query!(
        r#"SELECT id, nome, papel, ativo FROM usuarios WHERE id = $1"#,
        uid
    )
    .fetch_optional(pool)
    .await
    .map_err(erro_db)?;

    Ok(rec.filter(|u| u.ativo).map(|u| UsuarioSessao {
        id: u.id,
        nome: u.nome,
        papel: u.papel,
    }))
}

/// Auditoria best-effort de ações administrativas (não falha a requisição).
pub async fn auditar(pool: &PgPool, usuario_id: Option<Uuid>, acao: &str) {
    if let Err(e) = sqlx::query!(
        "INSERT INTO audit_log (usuario_id, acao) VALUES ($1, $2)",
        usuario_id,
        acao
    )
    .execute(pool)
    .await
    {
        tracing::error!(error = %e, "falha ao gravar auditoria");
    }
}

fn erro_db(e: sqlx::Error) -> AppError {
    tracing::error!(error = %e, "erro de banco em auth");
    AppError::Internal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_e_verificacao() {
        let h = gerar_hash("segredo123").unwrap();
        assert!(h.starts_with("$argon2id$"));
        assert!(verificar_senha(&h, "segredo123"));
        assert!(!verificar_senha(&h, "errada"));
        assert!(!verificar_senha("nao-e-hash", "x"));
    }

    /// Integração (requer Postgres em DATABASE_URL). Pula se não houver banco.
    #[tokio::test]
    async fn autentica_e_aplica_rate_limit() {
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
        let email = format!("teste-{n}@drinkup.local");
        let hash = gerar_hash("senhaForte!").unwrap();
        sqlx::query!(
            "INSERT INTO usuarios (nome, email, senha_hash, papel) VALUES ($1, $2, $3, 'editor')",
            "Teste",
            email,
            hash
        )
        .execute(&pool)
        .await
        .unwrap();

        // Senha correta autentica.
        assert!(autenticar(&pool, &email, "senhaForte!").await.is_ok());
        // Senhas erradas devolvem erro genérico (Unauthorized) até acionar o limite.
        for _ in 0..MAX_FALHAS {
            assert_eq!(
                autenticar(&pool, &email, "errada").await,
                Err(AppError::Unauthorized)
            );
        }
        // Excedido o limite, bloqueia mesmo com a senha certa.
        assert_eq!(
            autenticar(&pool, &email, "senhaForte!").await,
            Err(AppError::RateLimited)
        );

        // Limpeza (estado em memória + banco).
        limpar(&email.to_lowercase());
        sqlx::query!("DELETE FROM usuarios WHERE email = $1", email)
            .execute(&pool)
            .await
            .unwrap();
    }
}
