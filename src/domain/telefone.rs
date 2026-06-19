/// Aplica máscara de telefone brasileiro: mantém só dígitos (máx. 11) e formata
/// como "(00) 00000-0000". Formatação só para UX no cliente — o servidor
/// revalida o número ao processar.
#[must_use]
pub fn mascara_telefone(bruto: &str) -> String {
    let d: String = bruto.chars().filter(|c| c.is_ascii_digit()).take(11).collect();
    match d.len() {
        0 => String::new(),
        1..=2 => format!("({d}"),
        3..=7 => format!("({}) {}", &d[..2], &d[2..]),
        _ => format!("({}) {}-{}", &d[..2], &d[2..7], &d[7..]),
    }
}

#[cfg(test)]
mod tests {
    use super::mascara_telefone;

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
        assert_eq!(mascara_telefone("k4j4l9k9d8j1l2f4k3j6l6"), "(44) 99812-4366");
    }
}
