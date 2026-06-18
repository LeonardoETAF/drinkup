//! Envio de e-mail transacional via SMTP (server-only).
//!
//! Configure `SMTP_HOST`, `SMTP_PORT` (opcional, padrão 587), `SMTP_USER`,
//! `SMTP_PASSWORD` e `SMTP_FROM` no ambiente. Sem configuração, o conteúdo é
//! apenas registrado em log — útil em desenvolvimento, sem travar o fluxo.
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

type Erro = Box<dyn std::error::Error + Send + Sync>;

struct SmtpConfig {
    host: String,
    port: u16,
    user: String,
    senha: String,
    remetente: String,
}

fn env_nao_vazia(chave: &str) -> Option<String> {
    std::env::var(chave).ok().filter(|v| !v.trim().is_empty())
}

fn config() -> Option<SmtpConfig> {
    Some(SmtpConfig {
        host: env_nao_vazia("SMTP_HOST")?,
        port: env_nao_vazia("SMTP_PORT")
            .and_then(|p| p.parse().ok())
            .unwrap_or(587),
        user: env_nao_vazia("SMTP_USER")?,
        senha: env_nao_vazia("SMTP_PASSWORD")?,
        remetente: env_nao_vazia("SMTP_FROM")?,
    })
}

/// Envia o link de redefinição de senha. Nunca falha o fluxo: em caso de erro
/// (ou SMTP não configurado), registra em log e segue.
pub async fn enviar_link_redefinicao(destinatario: &str, nome: &str, link: &str) {
    let assunto = "Redefinição de senha — DRINK UP";
    let corpo = format!(
        "Olá, {nome}.\n\nRecebemos um pedido para redefinir a senha do seu acesso ao \
         painel DRINK UP. Para criar uma nova senha, acesse o link abaixo (válido por 1 hora):\n\n\
         {link}\n\nSe você não solicitou, ignore este e-mail — sua senha continua a mesma."
    );

    match config() {
        None => tracing::warn!(
            destinatario,
            link,
            "SMTP não configurado — link de redefinição apenas registrado em log"
        ),
        Some(cfg) => {
            if let Err(e) = enviar(&cfg, destinatario, assunto, &corpo).await {
                tracing::error!(error = %e, destinatario, link, "falha ao enviar e-mail — link em log");
            } else {
                tracing::info!(destinatario, "e-mail de redefinição enviado");
            }
        }
    }
}

async fn enviar(cfg: &SmtpConfig, para: &str, assunto: &str, corpo: &str) -> Result<(), Erro> {
    let email = Message::builder()
        .from(cfg.remetente.parse::<Mailbox>()?)
        .to(para.parse::<Mailbox>()?)
        .subject(assunto)
        .header(ContentType::TEXT_PLAIN)
        .body(corpo.to_string())?;

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&cfg.host)?
        .port(cfg.port)
        .credentials(Credentials::new(cfg.user.clone(), cfg.senha.clone()))
        .build();

    mailer.send(email).await?;
    Ok(())
}
