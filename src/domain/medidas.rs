//! Conversão entre milímetros (armazenado) e centímetros (exibido/editado).
//! Medidas lineares de produto são guardadas em mm inteiros e mostradas em cm.

/// Converte um texto em cm (aceita vírgula ou ponto) para mm inteiros, para
/// armazenar. Vazio ou inválido => `None`. Ex.: "20,1" -> Some(201).
#[must_use]
pub fn cm_para_mm(s: &str) -> Option<i32> {
    let t = s.trim().replace(',', ".");
    if t.is_empty() {
        return None;
    }
    let cm: f64 = t.parse().ok()?;
    Some((cm * 10.0).round() as i32)
}

/// Formata mm inteiros como cm com uma casa decimal (vírgula), sem unidade.
/// Ex.: 201 -> "20,1".
#[must_use]
pub fn mm_para_cm(mm: i32) -> String {
    format!("{},{}", mm / 10, (mm % 10).abs())
}

#[cfg(test)]
mod tests {
    use super::{cm_para_mm, mm_para_cm};

    #[test]
    fn cm_mm_conversao() {
        assert_eq!(cm_para_mm("20,1"), Some(201));
        assert_eq!(cm_para_mm("7.1"), Some(71));
        assert_eq!(cm_para_mm("6"), Some(60));
        assert_eq!(cm_para_mm("  "), None);
        assert_eq!(cm_para_mm("abc"), None);
        assert_eq!(mm_para_cm(201), "20,1");
        assert_eq!(mm_para_cm(71), "7,1");
        assert_eq!(mm_para_cm(60), "6,0");
    }
}
