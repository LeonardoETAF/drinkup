use leptos::prelude::*;

use crate::api::config::obter_contato;

const ICON_INSTAGRAM: &str = r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><rect x="3" y="3" width="18" height="18" rx="5"/><circle cx="12" cy="12" r="4"/><circle cx="17.5" cy="6.5" r="1" fill="currentColor" stroke="none"/></svg>"#;

const ICON_FACEBOOK: &str = r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M13 22v-8h3l1-4h-4V8c0-1.1.4-2 2-2h2V2.2C18.6 2.1 17.4 2 16 2c-3 0-5 1.8-5 5.2V10H8v4h3v8z"/></svg>"#;

const ICON_WHATSAPP: &str = r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M20 11.9a8 8 0 0 1-11.9 7L4 20l1.1-4A8 8 0 1 1 20 11.9zM12 5.5A6.4 6.4 0 0 0 6.6 15.5l-.6 2.2 2.3-.6A6.4 6.4 0 1 0 12 5.5zm3.6 8c-.2.5-1 .9-1.4 1-.4 0-.8.2-2.6-.6-2.2-1-3.6-3.2-3.7-3.4-.1-.2-.9-1.2-.9-2.2s.5-1.5.7-1.7c.2-.2.4-.2.5-.2h.4c.2 0 .3 0 .5.4l.6 1.5c0 .1 0 .3-.1.4l-.3.4-.3.3c-.1.1-.2.3-.1.5.1.2.6 1 1.3 1.6.9.8 1.6 1 1.8 1.1.2.1.3.1.5-.1l.6-.7c.2-.2.3-.2.5-.1l1.5.7c.2.1.3.2.4.3 0 .1 0 .6-.2 1z"/></svg>"#;

/// Rodapé do site público (fundo lima, conteúdo escuro). O envio da newsletter
/// será processado por server function na Fase 5.
#[component]
pub fn SiteFooter() -> impl IntoView {
    let info = Resource::new(|| (), |_| async move { obter_contato().await });
    view! {
        <footer class="site-footer">
            <div class="container site-footer__inner">
                <div class="site-footer__brand">
                    <div class="site-footer__logo">
                        <img
                            src="/brand/logo-footer.png"
                            alt="DRINK UP"
                            width="155"
                            height="34"
                        />
                    </div>
                    <p class="site-footer__brandinfo">
                        <Suspense fallback=|| ()>
                            {move || Suspend::new(async move {
                                let c = info.await.unwrap_or_default();
                                let nome = if c.nome_loja.trim().is_empty() {
                                    "DRINK UP".to_string()
                                } else {
                                    c.nome_loja
                                };
                                view! { {nome} <br/> {format!("CNPJ: {}", c.cnpj)} }
                            })}
                        </Suspense>
                    </p>
                    <div class="social">
                        <a href="#" aria-label="Facebook" inner_html=ICON_FACEBOOK></a>
                        <a href="#" aria-label="Instagram" inner_html=ICON_INSTAGRAM></a>
                        <a href="#" aria-label="WhatsApp" inner_html=ICON_WHATSAPP></a>
                    </div>
                </div>

                <div class="site-footer__news">
                    <h4>"Novidades"</h4>
                    <p class="site-footer__brandinfo">
                        "Receba promoções e lançamentos direto no seu WhatsApp."
                    </p>
                    <form class="newsletter">
                        <input
                            type="tel"
                            placeholder="(00) 00000-0000"
                            aria-label="Seu WhatsApp"
                        />
                        <button type="submit" class="btn btn--dark">"Quero receber"</button>
                    </form>
                </div>

                <div class="site-footer__legal">
                    <h4>"Legal"</h4>
                    <nav class="footer-links" aria-label="Links legais">
                        <a href="/politica-de-entrega">"Política de Entrega"</a>
                        <a href="/politica-de-privacidade">"Política de Privacidade"</a>
                        <a href="/termos-de-uso">"Termos de Uso"</a>
                    </nav>
                </div>
            </div>

            <div class="site-footer__bottom">
                <div class="container">"© 2026 DRINK UP · Todos os direitos reservados"</div>
            </div>
        </footer>
    }
}
