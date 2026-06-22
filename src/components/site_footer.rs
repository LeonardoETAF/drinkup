use leptos::prelude::*;

use crate::api::config::obter_contato;
use crate::domain::{link_whatsapp, mascara_telefone, Configuracoes};

const ICON_INSTAGRAM: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="3" y="3" width="18" height="18" rx="5"/><circle cx="12" cy="12" r="4"/><circle cx="17.2" cy="6.8" r="1.2" fill="currentColor" stroke="none"/></svg>"#;

const ICON_FACEBOOK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M22 12a10 10 0 1 0-11.56 9.88v-6.99H7.9V12h2.54V9.8c0-2.5 1.49-3.89 3.78-3.89 1.09 0 2.24.2 2.24.2v2.46h-1.26c-1.24 0-1.63.77-1.63 1.56V12h2.78l-.44 2.89h-2.34v6.99A10 10 0 0 0 22 12z"/></svg>"#;

const ICON_WHATSAPP: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 2.04c-5.5 0-9.96 4.46-9.96 9.96 0 1.76.46 3.48 1.34 5L2 22l5.1-1.34A9.93 9.93 0 0 0 12 22c5.5 0 9.96-4.46 9.96-9.96S17.5 2.04 12 2.04zm5.84 14.06c-.25.7-1.44 1.33-2 1.42-.51.08-1.16.11-1.87-.12-.43-.14-.98-.32-1.69-.62-2.97-1.28-4.9-4.27-5.05-4.47-.15-.2-1.21-1.61-1.21-3.07 0-1.46.77-2.18 1.04-2.48.27-.3.59-.37.79-.37l.57.01c.18 0 .43-.07.67.51.25.6.84 2.07.91 2.22.07.15.12.32.02.52-.1.2-.15.32-.3.5-.15.17-.31.39-.45.52-.15.15-.3.31-.13.61.17.3.77 1.27 1.65 2.06 1.13 1.01 2.09 1.32 2.39 1.47.3.15.47.13.65-.08.18-.2.74-.86.94-1.16.2-.3.4-.25.67-.15.27.1 1.72.81 2.02.96.3.15.5.22.57.35.07.12.07.72-.18 1.42z"/></svg>"#;

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

