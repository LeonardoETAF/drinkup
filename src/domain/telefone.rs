/// Aplica máscara de telefone brasileiro: mantém só dígitos (máx. 11) e formata
/// como "(00) 00000-0000". Formatação só para UX no cliente — o servidor
/// revalida o número ao processar.
#[must_use]
pub fn mascara_telefone(bruto: &str) -> String {
    let d: String = bruto
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(11)
        .collect();
    match d.len() {
        0 => String::new(),
        1..=2 => format!("({d}"),
        3..=7 => format!("({}) {}", &d[..2], &d[2..]),
        _ => format!("({}) {}-{}", &d[..2], &d[2..7], &d[7..]),
    }
}

/// Conta os dígitos de um telefone (ignora máscara/espaços).
#[must_use]
pub fn so_digitos(bruto: &str) -> usize {
    bruto.chars().filter(|c| c.is_ascii_digit()).count()
}

/// Telefone de contato válido: fixo (10) ou celular (11) dígitos. Usado nos
/// formulários gerais (ex.: orçamento), onde fixo é aceito.
#[must_use]
pub fn telefone_valido(bruto: &str) -> bool {
    matches!(so_digitos(bruto), 10 | 11)
}

/// Número de WhatsApp válido: exige celular com 11 dígitos (DDD + 9 + número).
/// Usado onde o canal é WhatsApp (newsletter/novidades).
#[must_use]
pub fn whatsapp_valido(bruto: &str) -> bool {
    so_digitos(bruto) == 11
}

/// Monta o link `wa.me` a partir do telefone cadastrado (só dígitos, com DDI 55).
/// Retorna `None` quando não há dígitos suficientes para um número válido.
#[must_use]
pub fn link_whatsapp(telefone: &str) -> Option<String> {
    let d: String = telefone.chars().filter(|c| c.is_ascii_digit()).collect();
    if d.len() < 10 {
        return None;
    }
    let numero = if d.starts_with("55") && d.len() >= 12 {
        d
    } else {
        format!("55{d}")
    };
    Some(format!("https://wa.me/{numero}"))
}

#[cfg(test)]
mod tests {
    use super::{link_whatsapp, mascara_telefone};

    #[test]
    fn mascara_remove_nao_digitos_e_formata() {
        assert_eq!(mascara_telefone(""), "");
        assert_eq!(mascara_telefone("abc"), "");
        assert_eq!(mascara_telefone("4"), "(4");
        assert_eq!(mascara_telefone("44"), "(44");
        assert_eq!(mascara_telefone("4498"), "(44) 98");
        assert_eq!(mascara_telefone("44998"), "(44) 998");
        assert_eq!(mascara_telefone("4499812"), "(44) 99812");
        assert_eq!(mascara_telefone("44998124366"), "(44) 99812-4366");
        // Letras intercaladas são descartadas; excedente é truncado em 11.
        assert_eq!(
            mascara_telefone("k4j4l9k9d8j1l2f4k3j6l6"),
            "(44) 99812-4366"
        );
    }

    #[test]
    fn link_whatsapp_adiciona_ddi_e_valida() {
        assert_eq!(link_whatsapp(""), None);
        assert_eq!(link_whatsapp("123"), None);
        assert_eq!(
            link_whatsapp("(44) 9 9812-4366"),
            Some("https://wa.me/5544998124366".to_string())
        );
        // Já com DDI 55 não duplica.
        assert_eq!(
            link_whatsapp("55 44 99812-4366"),
            Some("https://wa.me/5544998124366".to_string())
        );
    }
}
