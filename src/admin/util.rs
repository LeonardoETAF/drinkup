//! Helpers de apresentação do painel.

/// Iniciais para avatar (ex.: "Mariana Souza" -> "MS", "Administrador" -> "AD").
pub(crate) fn iniciais(nome: &str) -> String {
    let mut palavras = nome.split_whitespace();
    match (palavras.next(), palavras.next()) {
        (Some(a), Some(b)) => format!(
            "{}{}",
            a.chars().next().unwrap_or('?'),
            b.chars().next().unwrap_or('?')
        )
        .to_uppercase(),
        (Some(a), None) => a.chars().take(2).collect::<String>().to_uppercase(),
        _ => "?".to_string(),
    }
}

/// Rótulo legível para o status do lead.
pub(crate) fn status_label(status: &str) -> &'static str {
    match status {
        "novo" => "Novo",
        "em_atendimento" => "Em atendimento",
        "convertido" => "Convertido",
        "perdido" => "Perdido",
        _ => "—",
    }
}

/// Classe de badge conforme o status.
pub(crate) fn status_classe(status: &str) -> &'static str {
    match status {
        "novo" => "badge badge--lime",
        "em_atendimento" => "badge badge--cyan",
        "convertido" => "badge badge--green",
        "perdido" => "badge badge--muted",
        _ => "badge",
    }
}
