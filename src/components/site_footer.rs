use leptos::prelude::*;

use crate::api::config::obter_contato;
use crate::domain::{link_whatsapp, mascara_telefone, Configuracoes};

const ICON_INSTAGRAM: &str = r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><rect x="3" y="3" width="18" height="18" rx="5"/><circle cx="12" cy="12" r="4"/><circle cx="17.5" cy="6.5" r="1" fill="currentColor" stroke="none"/></svg>"#;

const ICON_FACEBOOK: &str = r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M13 22v-8h3l1-4h-4V8c0-1.1.4-2 2-2h2V2.2C18.6 2.1 17.4 2 16 2c-3 0-5 1.8-5 5.2V10H8v4h3v8z"/></svg>"#;

const ICON_WHATSAPP: &str = r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M20 11.9a8 8 0 0 1-11.9 7L4 20l1.1-4A8 8 0 1 1 20 11.9zM12 5.5A6.4 6.4 0 0 0 6.6 15.5l-.6 2.2 2.3-.6A6.4 6.4 0 1 0 12 5.5zm3.6 8c-.2.5-1 .9-1.4 1-.4 0-.8.2-2.6-.6-2.2-1-3.6-3.2-3.7-3.4-.1-.2-.9-1.2-.9-2.2s.5-1.5.7-1.7c.2-.2.4-.2.5-.2h.4c.2 0 .3 0 .5.4l.6 1.5c0 .1 0 .3-.1.4l-.3.4-.3.3c-.1.1-.2.3-.1.5.1.2.6 1 1.3 1.6.9.8 1.6 1 1.8 1.1.2.1.3.1.5-.1l.6-.7c.2-.2.3-.2.5-.1l1.5.7c.2.1.3.2.4.3 0 .1 0 .6-.2 1z"/></svg>"#;

/// Rodapé do site público (fundo lima, conteúdo escuro). O envio da newsletter
/// será processado por server function na Fase 5.
#[component]
pub fn SiteFooter() -> impl IntoView {
    let info = Resource::new(|| (), |_| async move { obter_contato().await });
    let (telefone, definir_telefone) = signal(String::new());
    // Feedback do envio: None = nada; Some((sucesso, texto)).
    let (mensagem, definir_mensagem) = signal::<Option<(bool, String)>>(None);

    // A inscrição é persistida no servidor (validação/dedupe lá).
    let inscrever = Action::new(|tel: &String| {
        let tel = tel.clone();
        async move { crate::api::novidades::inscrever_novidades(tel).await }
    });

    let enviar = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let bruto = telefone.get_untracked();
        let digitos = bruto.chars().filter(|c| c.is_ascii_digit()).count();
        if digitos == 11 {
            inscrever.dispatch(bruto);
        } else {
            definir_mensagem
                .set(Some((false, "Informe um número de WhatsApp válido.".to_string())));
        }
    };

    // Reflete o resultado da server function na mensagem de feedback.
    Effect::new(move |_| match inscrever.value().get() {
        Some(Ok(())) => {
            definir_telefone.set(String::new());
            definir_mensagem
                .set(Some((true, "Pronto! Em breve você receberá nossas novidades.".to_string())));
        }
        Some(Err(e)) => definir_mensagem.set(Some((false, e.to_string()))),
        None => {}
    });
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
                    <Suspense fallback=|| ()>
                        {move || Suspend::new(async move {
                            redes_sociais(info.await.unwrap_or_default())
                        })}
                    </Suspense>
                </div>

                <div class="site-footer__news">
                    <h4>"Novidades"</h4>
                    <p class="site-footer__brandinfo">
                        "Receba promoções e lançamentos direto no seu WhatsApp."
                    </p>
                    <form class="newsletter" on:submit=enviar>
                        <input
                            type="tel"
                            inputmode="numeric"
                            maxlength="15"
                            placeholder="(00) 00000-0000"
                            aria-label="Seu WhatsApp"
                            prop:value=telefone
                            on:input=move |ev| {
                                definir_telefone.set(mascara_telefone(&event_target_value(&ev)));
                                definir_mensagem.set(None);
                            }
                        />
                        <button type="submit" class="btn btn--dark">"Quero receber"</button>
                    </form>
                    {move || {
                        mensagem
                            .get()
                            .map(|(sucesso, texto)| {
                                let classe = if sucesso {
                                    "newsletter__msg newsletter__msg--ok"
                                } else {
                                    "newsletter__msg newsletter__msg--erro"
                                };
                                view! {
                                    <p class=classe role="status" aria-live="polite">
                                        {texto}
                                    </p>
                                }
                            })
                    }}
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

/// Botões de redes sociais com os links cadastrados. Facebook/Instagram vêm das
/// configurações; o WhatsApp é montado a partir do telefone. Sem destino, o
/// botão fica inerte ("#").
fn redes_sociais(c: Configuracoes) -> impl IntoView {
    // Mostra cada rede só se estiver ativa e com destino configurado.
    let fb = (c.facebook_ativo && !c.facebook.trim().is_empty()).then_some(c.facebook);
    let ig = (c.instagram_ativo && !c.instagram.trim().is_empty()).then_some(c.instagram);
    let wa = c
        .whatsapp_ativo
        .then(|| link_whatsapp(&c.telefone))
        .flatten();
    view! {
        <div class="social">
            {fb
                .map(|u| {
                    view! {
                        <a
                            href=u
                            aria-label="Facebook"
                            target="_blank"
                            rel="noopener noreferrer"
                            inner_html=ICON_FACEBOOK
                        ></a>
                    }
                })}
            {ig
                .map(|u| {
                    view! {
                        <a
                            href=u
                            aria-label="Instagram"
                            target="_blank"
                            rel="noopener noreferrer"
                            inner_html=ICON_INSTAGRAM
                        ></a>
                    }
                })}
            {wa
                .map(|u| {
                    view! {
                        <a
                            href=u
                            aria-label="WhatsApp"
                            target="_blank"
                            rel="noopener noreferrer"
                            inner_html=ICON_WHATSAPP
                        ></a>
                    }
                })}
        </div>
    }
}

