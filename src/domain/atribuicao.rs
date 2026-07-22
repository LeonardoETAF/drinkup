//! Handoff de atribuição site → WhatsApp: tipo compartilhado cliente/servidor.
//!
//! O navegador monta este registro no clique do WhatsApp e o envia ao nosso
//! servidor, que repassa à *edge function* do CRM. Sem lógica sensível aqui —
//! a chave da edge e o envio ficam em `server::atribuicao`.
use serde::{Deserialize, Serialize};

/// Marca esperada pela edge (o CRM usa para rotear o registro).
pub const MARCA: &str = "DRINKUP";

/// Identificadores do clique. `fbc`/`fbp` trafegam **sem hash** — o Meta os usa
/// crus, e quem faz hash de telefone/e-mail é o CRM, depois.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Handoff {
    pub token: String,
    pub brand: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbclid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id_contact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_url: Option<String>,
}

impl Handoff {
    /// Token no formato exigido pela edge: `DU-` + 6 a 8 caracteres `A-Z0-9`.
    #[must_use]
    pub fn token_valido(token: &str) -> bool {
        let Some(sufixo) = token.strip_prefix("DU-") else {
            return false;
        };
        (6..=8).contains(&sufixo.len())
            && sufixo
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    }

    /// Sem nenhum identificador não há o que atribuir — a edge devolveria
    /// `missing_attribution`. Verificado nas duas pontas.
    #[must_use]
    pub fn tem_atribuicao(&self) -> bool {
        [&self.fbc, &self.fbp, &self.fbclid]
            .iter()
            .any(|v| v.as_ref().is_some_and(|s| !s.trim().is_empty()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Handoff {
        Handoff {
            token: "DU-A1B2C3".into(),
            brand: MARCA.into(),
            fbc: Some("fb.1.1720000000000.IwAR123".into()),
            ..Default::default()
        }
    }

    #[test]
    fn valida_formato_do_token() {
        assert!(Handoff::token_valido("DU-A1B2C3"));
        assert!(Handoff::token_valido("DU-ABCDEFGH"));
        assert!(!Handoff::token_valido("DU-a1b2c3"), "minúsculas não passam");
        assert!(!Handoff::token_valido("DU-ABC"), "curto demais");
        assert!(!Handoff::token_valido("DU-ABCDEFGHI"), "longo demais");
        assert!(!Handoff::token_valido("XX-ABCDEF"), "prefixo errado");
        assert!(!Handoff::token_valido("DU-ABC_DEF"), "caractere inválido");
    }

    #[test]
    fn exige_ao_menos_um_identificador() {
        assert!(base().tem_atribuicao());

        let so_fbp = Handoff {
            fbc: None,
            fbp: Some("fb.1.1720000000000.987".into()),
            ..base()
        };
        assert!(so_fbp.tem_atribuicao());

        let vazio = Handoff {
            fbc: None,
            ..base()
        };
        assert!(!vazio.tem_atribuicao());

        let branco = Handoff {
            fbc: Some("   ".into()),
            ..base()
        };
        assert!(!branco.tem_atribuicao(), "string em branco não conta");
    }
}
