//! Validação e gravação de imagens enviadas. Server-only.
//! Valida o tipo pelo conteúdo real (magic bytes) e o tamanho; nunca confia
//! no nome ou no content-type enviado pelo cliente. Gera variantes responsivas
//! (para `srcset`) quando o formato permite.
use std::io::Cursor;

use argon2::password_hash::rand_core::{OsRng, RngCore};
use image::ImageFormat;

use crate::error::AppError;

/// Diretório (relativo ao processo) onde as imagens são gravadas e servidas em `/uploads`.
pub const DIR_UPLOADS: &str = "uploads";
const MAX_BYTES: usize = 5 * 1024 * 1024; // 5 MB
/// Larguras das variantes responsivas (criadas só quando menores que o original).
const LARGURAS: [u32; 3] = [400, 800, 1200];

/// Detecta a extensão a partir dos bytes iniciais (apenas formatos aceitos).
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

/// Valida e grava a imagem (e suas variantes). Retorna a URL pública; quando há
/// variantes, anexa o manifesto `?srcset=<larguras>` para o cliente montar o `srcset`.
pub async fn salvar_imagem(bytes: &[u8]) -> Result<String, AppError> {
    if bytes.is_empty() || bytes.len() > MAX_BYTES {
        return Err(AppError::Validation);
    }
    let ext = detectar_extensao(bytes).ok_or(AppError::Validation)?;
    let id = token();
    let nome = format!("{id}.{ext}");

    tokio::fs::create_dir_all(DIR_UPLOADS)
        .await
        .map_err(erro_io)?;
    tokio::fs::write(format!("{DIR_UPLOADS}/{nome}"), bytes)
        .await
        .map_err(erro_io)?;

    let url = format!("/uploads/{nome}");

    // Variantes responsivas: best-effort e apenas para jpg/png. Erros não
    // interrompem o upload (a imagem original já foi gravada).
    let larguras = if ext == "jpg" || ext == "png" {
        gerar_variantes(bytes, &id, ext).await
    } else {
        Vec::new()
    };

    if larguras.is_empty() {
        Ok(url)
    } else {
        let csv: Vec<String> = larguras.iter().map(u32::to_string).collect();
        Ok(format!("{url}?srcset={}", csv.join(",")))
    }
}

/// Gera as variantes reduzidas e devolve as larguras efetivamente criadas.
async fn gerar_variantes(bytes: &[u8], id: &str, ext: &str) -> Vec<u32> {
    let dados = bytes.to_vec();
    let ext_owned = ext.to_string();

    // Decodificar/redimensionar é trabalho de CPU → thread dedicada.
    let resultado = tokio::task::spawn_blocking(move || redimensionar(&dados, &ext_owned)).await;
    let imagens = match resultado {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => {
            tracing::warn!(error = %e, "falha ao gerar variantes de imagem");
            return Vec::new();
        }
        Err(e) => {
            tracing::warn!(error = %e, "thread de variantes falhou");
            return Vec::new();
        }
    };

    let mut criadas = Vec::new();
    for (largura, dados) in imagens {
        let caminho = format!("{DIR_UPLOADS}/{id}-{largura}.{ext}");
        if tokio::fs::write(&caminho, dados).await.is_ok() {
            criadas.push(largura);
        }
    }
    criadas
}

/// Redimensiona (preservando proporção) para cada largura menor que a original.
fn redimensionar(bytes: &[u8], ext: &str) -> Result<Vec<(u32, Vec<u8>)>, image::ImageError> {
    let original = image::load_from_memory(bytes)?;
    let largura_orig = original.width();
    let mut saidas = Vec::new();

    for largura in LARGURAS.into_iter().filter(|&l| l < largura_orig) {
        let menor = original.resize(largura, u32::MAX, image::imageops::FilterType::Lanczos3);
        let mut buf = Cursor::new(Vec::new());
        if ext == "png" {
            menor.write_to(&mut buf, ImageFormat::Png)?;
        } else {
            // JPEG não tem canal alfa: converte para RGB e usa qualidade 82.
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 82);
            menor.to_rgb8().write_with_encoder(encoder)?;
        }
        saidas.push((largura, buf.into_inner()));
    }
    Ok(saidas)
}

fn erro_io(e: std::io::Error) -> AppError {
    tracing::error!(error = %e, "falha ao gravar imagem");
    AppError::Internal
}
