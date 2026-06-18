//! Validação e gravação de imagens enviadas. Server-only.
//! Valida o tipo pelo conteúdo real (magic bytes) e o tamanho; nunca confia
//! no nome ou no content-type enviado pelo cliente.
use argon2::password_hash::rand_core::{OsRng, RngCore};

use crate::error::AppError;

/// Diretório (relativo ao processo) onde as imagens são gravadas e servidas em `/uploads`.
pub const DIR_UPLOADS: &str = "uploads";
const MAX_BYTES: usize = 5 * 1024 * 1024; // 5 MB

/// Detecta a extensão a partir dos bytes iniciais (apenas formatos de imagem aceitos).
fn detectar_extensao(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some("jpg")
    } else if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
        Some("png")
    } else if bytes.len() > 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        Some("webp")
    } else {
        None
    }
}

/// Nome de arquivo aleatório (hex) — evita colisões e nomes forjados.
fn token() -> String {
    let mut b = [0u8; 16];
    OsRng.fill_bytes(&mut b);
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Valida e grava a imagem; retorna a URL pública (`/uploads/<arquivo>`).
pub async fn salvar_imagem(bytes: &[u8]) -> Result<String, AppError> {
    if bytes.is_empty() || bytes.len() > MAX_BYTES {
        return Err(AppError::Validation);
    }
    let ext = detectar_extensao(bytes).ok_or(AppError::Validation)?;
    let nome = format!("{}.{ext}", token());

    tokio::fs::create_dir_all(DIR_UPLOADS)
        .await
        .map_err(erro_io)?;
    tokio::fs::write(format!("{DIR_UPLOADS}/{nome}"), bytes)
        .await
        .map_err(erro_io)?;

    Ok(format!("/uploads/{nome}"))
}

fn erro_io(e: std::io::Error) -> AppError {
    tracing::error!(error = %e, "falha ao gravar imagem");
    AppError::Internal
}
