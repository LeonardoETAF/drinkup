//! Página institucional do aplicativo OAuth "DrinkUp" — exigida pela verificação
//! de marca do Google Cloud (Google Ads API). Pública, sem login.
//!
//! O nome "DrinkUp" precisa aparecer em destaque no título: é ele que o Google
//! confere contra o nome do app OAuth.
use leptos::prelude::*;

use crate::components::Seo;

/// Uso dos dados acessados nas APIs (Google Ads, Analytics e Meta).
const USO_DOS_DADOS: &[&str] = &[
    "Acessamos métricas de desempenho das nossas campanhas (campanhas, grupos de anúncios, \
     palavras-chave, conversões e relatórios) para gerar análises e otimizações internas.",
    "O uso é exclusivamente interno da nossa equipe (colaboradores e prestadores atuando em \
     nosso nome).",
    "Não vendemos, não compartilhamos e não transferimos esses dados nem o acesso à API a \
     terceiros.",
    "Toda alteração nas contas é revisada e aprovada manualmente antes de ser executada.",
];

#[component]
pub fn IntegracaoPage() -> impl IntoView {
    view! {
        <Seo
            titulo="DrinkUp — Plataforma de Marketing e Análise"
            descricao="DrinkUp: página oficial da integração interna de marketing e análise da \
            Drink Up (drinkup.com.br), usada para gerenciar as nossas próprias contas de Google \
            Ads, Google Analytics e Meta."
            caminho="/integracao"
        />

        <section class="legal-hero">
            <div class="container">
                <p class="integracao__kicker">"Plataforma interna"</p>
                // Sem uppercase (ver `_legal.scss`): o nome tem de aparecer grafado
                // exatamente "DrinkUp", como o app OAuth registrado no Google.
                <h1 class="legal-hero__title integracao__titulo">"DrinkUp"</h1>
                <p class="legal-hero__sub">
                    "A plataforma de marketing e análise da Drink Up — usada pela nossa equipe \
                     para gerenciar e analisar as nossas próprias contas de anúncios."
                </p>
            </div>
        </section>

        <article class="legal container">
            <section class="legal__sec">
                <h2 class="legal__sec-title">"Sobre este aplicativo"</h2>
                <p class="legal__clausula">
                    "Esta é a página oficial do aplicativo DrinkUp, a integração interna da Drink \
                     Up — empresa de copos, garrafas e baldes personalizados (in-mold label / full \
                     color 360°) sediada em Maringá-PR, com site em "
                    <a href="https://drinkup.com.br">"drinkup.com.br"</a>
                    "."
                </p>
                <p class="legal__clausula">
                    "O aplicativo DrinkUp conecta-se às plataformas de anúncios e análise — Google \
                     Ads, Google Analytics e Meta — para gerenciar e medir o desempenho das nossas \
                     próprias contas de publicidade."
                </p>
            </section>

            <section class="legal__sec">
                <h2 class="legal__sec-title">"Como usamos os dados"</h2>
                <ul class="legal__lista">
                    {USO_DOS_DADOS
                        .iter()
                        .map(|item| view! { <li class="legal__item">{*item}</li> })
                        .collect_view()}
                </ul>
                <p class="integracao__nota">
                    "O acesso e o uso de dados seguem as políticas de dados de usuário das APIs do \
                     Google, incluindo os requisitos de Uso Limitado quando aplicáveis."
                </p>
            </section>

            <section class="legal__sec">
                <h2 class="legal__sec-title">"Privacidade e Termos"</h2>
                <p class="legal__clausula">"Consulte nossos documentos oficiais:"</p>
                <div class="integracao__links">
                    <a class="btn btn--primary" href="/politica-de-privacidade">
                        "Política de Privacidade"
                    </a>
                    <a class="btn btn--ghost" href="/termos-de-uso">"Termos de Uso"</a>
                </div>
            </section>

            <section class="legal__sec">
                <h2 class="legal__sec-title">"Contato"</h2>
                <p class="legal__clausula">
                    "DrinkUp · Maringá-PR, Brasil · "
                    <a href="https://drinkup.com.br">"drinkup.com.br"</a>
                    " · "
                    <a href="mailto:drinkup.mkt@gmail.com">"drinkup.mkt@gmail.com"</a>
                </p>
            </section>
        </article>
    }
}
