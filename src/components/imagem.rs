//! Helper de imagem responsiva (cliente e servidor). Puro, sem segredos.

/// A partir da URL armazenada (possivelmente com manifesto `?srcset=larguras`),
/// devolve `(src, srcset_opcional)`.
///
/// Ex.: `"/uploads/ab.jpg?srcset=400,800"` →
/// `("/uploads/ab.jpg", Some("/uploads/ab-400.jpg 400w, /uploads/ab-800.jpg 800w"))`.
pub fn responsiva(url: &str) -> (String, Option<String>) {
    let Some((base, larguras)) = url.split_once("?srcset=") else {
        return (url.to_string(), None);
    };
    let Some((stem, ext)) = base.rsplit_once('.') else {
        return (base.to_string(), None);
    };
    let candidatos: Vec<String> = larguras
        .split(',')
        .filter_map(|l| l.trim().parse::<u32>().ok())
        .map(|l| format!("{stem}-{l}.{ext} {l}w"))
        .collect();
    let srcset = (!candidatos.is_empty()).then(|| candidatos.join(", "));
    (base.to_string(), srcset)
}

#[cfg(test)]
mod tests {
    use super::responsiva;

    #[test]
    fn sem_manifesto_mantem_url() {
        let (src, srcset) = responsiva("/uploads/a.jpg");
        assert_eq!(src, "/uploads/a.jpg");
        assert!(srcset.is_none());
    }

    #[test]
    fn com_manifesto_monta_srcset() {
        let (src, srcset) = responsiva("/uploads/ab.jpg?srcset=400,800");
        assert_eq!(src, "/uploads/ab.jpg");
        assert_eq!(
            srcset.as_deref(),
            Some("/uploads/ab-400.jpg 400w, /uploads/ab-800.jpg 800w")
        );
    }
}
