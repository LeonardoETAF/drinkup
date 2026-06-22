use leptos::prelude::*;

use crate::components::Seo;

/// Bloco de conteúdo de uma seção da política.
enum Bloco {
    /// Parágrafo corrido.
    P(&'static str),
    /// Item com rótulo em negrito seguido de descrição.
    Item(&'static str, &'static str),
    /// Item de lista com marcador.
    Li(&'static str),
}

use Bloco::{Item, Li, P};

/// Seção numerada: (título, blocos).
type Secao = (&'static str, &'static [Bloco]);

/// Texto introdutório (antes das seções numeradas).
const PREAMBULO: &[&str] = &[
    "A DrinkUp, pessoa jurídica de direito privado, inscrita no CNPJ sob o nº \
     45.185.460/0001-70, com sede na Av. Pedro Taques, 5217 - Lot. Sumaré, Maringá - PR, \
     CEP 87035-591, doravante denominada DrinkUp, nós ou nossa empresa, valoriza a \
     privacidade, a segurança e a transparência no tratamento dos dados pessoais de seus \
     clientes, parceiros, fornecedores, visitantes, usuários, representantes de empresas e \
     demais pessoas que interagem com nossos canais de atendimento.",
    "Esta Política de Privacidade explica como coletamos, utilizamos, armazenamos, \
     compartilhamos e protegemos os dados pessoais tratados pela DrinkUp em seus canais \
     digitais e comerciais, incluindo WhatsApp, redes sociais, site, telefone, e-mail, \
     formulários, plataformas de orçamento, atendimento e demais meios utilizados para a \
     oferta de nossos produtos e serviços.",
    "Ao entrar em contato conosco, solicitar orçamento, realizar pedidos, enviar artes para \
     personalização, contratar nossos produtos, interagir com nossas redes sociais ou \
     utilizar nossos canais de atendimento, você declara estar ciente das práticas descritas \
     nesta Política.",
    "A DrinkUp realiza o tratamento de dados pessoais em conformidade com a Lei Geral de \
     Proteção de Dados Pessoais, Lei nº 13.709/2018, conhecida como LGPD, e demais normas \
     aplicáveis.",
];

const SECOES: &[Secao] = &[
    (
        "1. Dados pessoais que tratamos",
        &[
            P("A DrinkUp poderá coletar e tratar dados pessoais fornecidos diretamente por você \
               ou gerados durante sua interação com nossos canais de atendimento, orçamento, \
               compra, produção e pós-venda."),
            P("Podemos tratar os seguintes dados:"),
            Item(
                "Dados de identificação e contato",
                "nome completo, telefone, WhatsApp, e-mail, CPF, RG, cargo, empresa, razão \
                 social, CNPJ, inscrição estadual, endereço de cobrança e endereço de entrega.",
            ),
            Item(
                "Dados comerciais e de pedido",
                "produtos solicitados, quantidades, modelos, cores, tipos de personalização, \
                 informações de orçamento, histórico de compras, status do pedido, notas \
                 fiscais, comprovantes, informações de entrega, retirada, troca, devolução e \
                 atendimento pós-venda.",
            ),
            Item(
                "Dados para personalização de produtos",
                "artes enviadas, logotipos, nomes, frases, imagens, fotos, referências visuais, \
                 brasões, símbolos, arquivos, marcas, materiais gráficos e demais conteúdos \
                 enviados para criação, ajuste, aprovação e produção de produtos personalizados.",
            ),
            Item(
                "Dados de pagamento",
                "informações necessárias para confirmação de pagamentos, identificação de \
                 transações, comprovantes, dados bancários para reembolso, quando aplicável, e \
                 informações processadas por instituições financeiras ou intermediadores de \
                 pagamento.",
            ),
            Item(
                "Dados de navegação e interação digital",
                "endereço IP, data e horário de acesso, páginas visitadas, origem do acesso, \
                 dispositivo utilizado, tipo de navegador, cookies e outras tecnologias \
                 semelhantes, quando aplicável.",
            ),
            Item(
                "Dados de atendimento",
                "mensagens enviadas por WhatsApp, redes sociais, telefone, e-mail ou outros \
                 canais oficiais, incluindo dúvidas, solicitações, reclamações, aprovações de \
                 arte, negociações comerciais, suporte e resolução de problemas.",
            ),
            P("A DrinkUp não tem como finalidade coletar dados pessoais sensíveis. Caso você \
               envie espontaneamente informações dessa natureza em artes, imagens, documentos, \
               mensagens ou arquivos, elas serão tratadas apenas quando forem necessárias para \
               atender à solicitação realizada, ou poderão ser descartadas quando não forem \
               essenciais."),
            P("Quando o cliente enviar dados, imagens, nomes, fotos, logotipos, marcas ou \
               informações de terceiros para personalização de produtos, declara possuir \
               autorização para esse uso, especialmente em casos que envolvam crianças, \
               adolescentes, colaboradores, convidados, instituições, empresas ou marcas \
               registradas."),
        ],
    ),
    (
        "2. Finalidades do uso dos dados pessoais",
        &[
            P("A DrinkUp poderá utilizar os dados pessoais para as seguintes finalidades:"),
            Li("Realizar atendimento comercial, responder dúvidas, preparar orçamentos e dar \
                continuidade às negociações."),
            Li("Processar pedidos, emitir notas fiscais, confirmar pagamentos, organizar \
                produção, personalização, embalagem, faturamento, retirada ou envio dos \
                produtos."),
            Li("Criar, revisar, ajustar, validar e aprovar artes enviadas ou solicitadas pelo \
                cliente."),
            Li("Entrar em contato sobre orçamento, pedido, aprovação de arte, produção, \
                pagamento, entrega, retirada, troca, devolução, suporte e demais informações \
                relacionadas ao atendimento."),
            Li("Gerenciar cadastros de clientes, revendedores, representantes, fornecedores, \
                parceiros comerciais e empresas atendidas."),
            Li("Cumprir obrigações legais, fiscais, contábeis, regulatórias e exigências de \
                autoridades competentes."),
            Li("Prevenir fraudes, proteger direitos da DrinkUp, resolver conflitos, garantir \
                segurança nas transações e preservar registros necessários."),
            Li("Melhorar nossos produtos, serviços, atendimento, canais digitais, campanhas, \
                processos internos e experiência de compra."),
            Li("Enviar comunicações comerciais, novidades, promoções, conteúdos, campanhas e \
                materiais de marketing, respeitando a legislação aplicável e a possibilidade de \
                cancelamento pelo titular."),
            Li("Realizar análises estatísticas, relatórios internos, estudos de desempenho e \
                melhoria de comunicação, sempre que possível de forma agregada ou anonimizada."),
        ],
    ),
    (
        "3. Bases legais para o tratamento dos dados",
        &[
            P("O tratamento de dados pessoais pela DrinkUp poderá ocorrer com base nas seguintes \
               hipóteses previstas na LGPD:"),
            Item(
                "Execução de contrato ou procedimentos preliminares",
                "quando os dados forem necessários para realizar orçamento, pedido, \
                 personalização, pagamento, entrega e atendimento ao cliente.",
            ),
            Item(
                "Cumprimento de obrigação legal ou regulatória",
                "quando for necessário emitir nota fiscal, manter registros contábeis, fiscais, \
                 financeiros ou atender exigências legais.",
            ),
            Item(
                "Legítimo interesse",
                "quando a DrinkUp precisar tratar dados para melhorar serviços, manter \
                 relacionamento comercial, proteger direitos, prevenir fraudes, realizar \
                 comunicações relacionadas ao atendimento ou aprimorar seus processos, sempre \
                 respeitando os direitos dos titulares.",
            ),
            Item(
                "Consentimento",
                "quando necessário para finalidades específicas, como determinadas comunicações \
                 promocionais, uso de cookies não essenciais ou outras situações em que a \
                 autorização do titular seja exigida.",
            ),
            Item(
                "Exercício regular de direitos",
                "quando os dados forem necessários em processos judiciais, administrativos ou \
                 arbitrais.",
            ),
            Item("Proteção do crédito", "quando aplicável e permitido pela legislação."),
        ],
    ),
    (
        "4. Compartilhamento de dados pessoais",
        &[
            P("A DrinkUp não vende dados pessoais."),
            P("Os dados pessoais poderão ser compartilhados somente quando necessário para a \
               execução dos serviços, operação do negócio, cumprimento de obrigações legais ou \
               proteção de direitos."),
            P("O compartilhamento poderá ocorrer com:"),
            Item(
                "Empresas do mesmo grupo econômico ou parceiros internos de operação",
                "quando necessário para atendimento, produção, faturamento, logística, suporte, \
                 gestão comercial ou relacionamento com o cliente.",
            ),
            Item(
                "Prestadores de serviços e fornecedores",
                "incluindo empresas de tecnologia, hospedagem, armazenamento em nuvem, sistemas \
                 de gestão, CRM, automação, suporte técnico, atendimento, marketing, \
                 contabilidade, emissão fiscal, meios de pagamento e análise antifraude.",
            ),
            Item(
                "Transportadoras, Correios e operadores logísticos",
                "para viabilizar entrega, rastreamento, retirada, reentrega ou resolução de \
                 ocorrências relacionadas ao envio dos produtos.",
            ),
            Item(
                "Parceiros de produção, acabamento ou personalização",
                "quando necessário para viabilizar o pedido contratado, observando o limite das \
                 informações necessárias para a execução do serviço.",
            ),
            Item(
                "Autoridades públicas, órgãos reguladores, fiscais, judiciais ou administrativos",
                "quando houver obrigação legal, solicitação válida ou necessidade de defesa de \
                 direitos da DrinkUp.",
            ),
            Item(
                "Plataformas de mídia, publicidade e análise digital",
                "quando utilizadas para mensuração de campanhas, remarketing, anúncios, \
                 estatísticas ou melhoria da comunicação, respeitando as configurações de \
                 privacidade e consentimento aplicáveis.",
            ),
            P("Quando houver utilização de ferramentas digitais com armazenamento ou \
               processamento fora do Brasil, poderá ocorrer transferência internacional de \
               dados. Nesses casos, a DrinkUp buscará adotar medidas adequadas de proteção, \
               conforme a legislação aplicável."),
        ],
    ),
    (
        "5. Retenção e armazenamento dos dados",
        &[
            P("Os dados pessoais serão armazenados pelo tempo necessário para cumprir as \
               finalidades descritas nesta Política, incluindo atendimento, orçamento, produção, \
               personalização, entrega, pós-venda, obrigações fiscais, obrigações legais, defesa \
               de direitos e manutenção de registros comerciais."),
            P("Artes, arquivos, imagens e informações enviadas para personalização poderão ser \
               mantidos pelo período necessário para produção, conferência, reimpressão, \
               atendimento pós-venda, histórico do pedido ou cumprimento de obrigações legais."),
            P("Quando os dados não forem mais necessários, poderão ser eliminados, anonimizados \
               ou mantidos apenas nas hipóteses permitidas pela legislação."),
        ],
    ),
    (
        "6. Segurança dos dados",
        &[
            P("A DrinkUp adota medidas técnicas e organizacionais para proteger os dados \
               pessoais contra acessos não autorizados, perda, alteração, uso indevido, \
               divulgação indevida, destruição ou qualquer forma de tratamento inadequado."),
            P("Essas medidas podem incluir controle de acesso, restrição de informações apenas a \
               colaboradores e prestadores que precisam utilizá-las, armazenamento seguro, uso \
               de sistemas protegidos, organização interna de documentos e orientação da equipe \
               sobre confidencialidade."),
            P("Apesar dos esforços de segurança, nenhum sistema físico ou digital é totalmente \
               livre de riscos. Por isso, recomendamos que os usuários evitem enviar informações \
               desnecessárias, mantenham seus dados atualizados e comuniquem a DrinkUp caso \
               identifiquem qualquer suspeita de uso indevido de seus dados."),
        ],
    ),
    (
        "7. Menores de 18 anos",
        &[
            P("Os canais de compra, orçamento e contratação da DrinkUp são destinados \
               preferencialmente a pessoas maiores de 18 anos ou representantes legais de \
               empresas."),
            P("Quando houver envio de nomes, fotos, imagens, frases, turmas, instituições ou \
               demais dados de crianças e adolescentes para personalização de produtos, o \
               cliente declara possuir autorização dos pais ou responsáveis legais para esse \
               uso."),
            P("Caso a DrinkUp identifique o tratamento indevido de dados de menores de idade sem \
               autorização adequada, poderá solicitar comprovação de autorização, interromper o \
               atendimento relacionado ou excluir as informações, conforme o caso."),
        ],
    ),
    (
        "8. Comunicações da DrinkUp",
        &[
            P("A DrinkUp poderá entrar em contato com você por meio de WhatsApp, redes sociais, \
               telefone, e-mail ou outros canais informados durante o atendimento, para tratar \
               de orçamentos, pedidos, aprovação de arte, produção, pagamento, entrega, \
               retirada, suporte, troca, devolução e resolução de solicitações."),
            P("Também poderemos enviar comunicações comerciais, campanhas, novidades, promoções, \
               conteúdos e informações sobre produtos por WhatsApp, redes sociais ou outros \
               canais oficiais da DrinkUp."),
            P("Caso não queira mais receber comunicações promocionais, você poderá solicitar o \
               cancelamento do recebimento por meio dos próprios canais de atendimento da \
               DrinkUp."),
            P("As comunicações necessárias para andamento de pedidos, orçamentos, entregas, \
               pagamentos, suporte ou cumprimento de obrigações legais poderão continuar sendo \
               enviadas enquanto forem necessárias para a prestação do serviço."),
        ],
    ),
    (
        "9. Direitos dos titulares de dados pessoais",
        &[
            P("Nos termos da LGPD, o titular dos dados pessoais poderá solicitar à DrinkUp:"),
            Li("Confirmação da existência de tratamento de seus dados pessoais."),
            Li("Acesso aos dados pessoais tratados pela empresa."),
            Li("Correção de dados incompletos, inexatos ou desatualizados."),
            Li("Anonimização, bloqueio ou eliminação de dados desnecessários, excessivos ou \
                tratados em desconformidade com a LGPD."),
            Li("Portabilidade dos dados a outro fornecedor de produtos ou serviços, quando \
                aplicável e conforme regulamentação da autoridade competente."),
            Li("Informações sobre o compartilhamento de dados com terceiros."),
            Li("Revogação do consentimento, quando o tratamento tiver sido baseado nessa \
                autorização."),
            Li("Eliminação dos dados pessoais tratados com base no consentimento, observadas as \
                hipóteses legais de conservação."),
            Li("Oposição ao tratamento de dados em caso de descumprimento da legislação \
                aplicável."),
            Li("Revisão de decisões tomadas unicamente com base em tratamento automatizado de \
                dados pessoais, caso existam."),
            Li("Peticionamento perante a Autoridade Nacional de Proteção de Dados ou organismos \
                de defesa do consumidor, quando entender necessário."),
            P("Para exercer seus direitos, o titular poderá entrar em contato com a DrinkUp por \
               meio dos canais oficiais indicados ao final desta Política."),
            P("A DrinkUp poderá solicitar informações adicionais para confirmar a identidade do \
               solicitante e garantir a segurança dos dados antes de atender determinadas \
               solicitações."),
        ],
    ),
    (
        "10. Cookies e tecnologias de rastreamento",
        &[
            P("A DrinkUp poderá utilizar cookies e tecnologias semelhantes em seu site e canais \
               digitais para melhorar a experiência de navegação, garantir o funcionamento \
               adequado da plataforma, lembrar preferências, analisar desempenho, entender \
               interações dos visitantes e apresentar campanhas mais relevantes."),
            P("Os cookies podem ser classificados como:"),
            Item(
                "Cookies necessários",
                "essenciais para o funcionamento do site e de recursos básicos de segurança.",
            ),
            Item(
                "Cookies funcionais",
                "ajudam a lembrar preferências do usuário e melhorar a experiência de navegação.",
            ),
            Item(
                "Cookies analíticos",
                "permitem entender como os visitantes interagem com o site, auxiliando na \
                 melhoria de conteúdo, desempenho e usabilidade.",
            ),
            Item(
                "Cookies de publicidade",
                "podem ser utilizados para apresentar anúncios e campanhas mais relevantes, \
                 inclusive por meio de plataformas de mídia, remarketing e redes sociais.",
            ),
            P("O usuário poderá configurar seu navegador para bloquear, limitar ou excluir \
               cookies. No entanto, essa ação poderá afetar algumas funcionalidades do site."),
            P("Quando exigido pela legislação aplicável, a DrinkUp solicitará consentimento para \
               o uso de cookies não essenciais, permitindo que o usuário gerencie suas \
               preferências."),
        ],
    ),
    (
        "11. Redes sociais e plataformas de terceiros",
        &[
            P("A DrinkUp utiliza redes sociais e plataformas digitais para divulgação de \
               produtos, atendimento, relacionamento com clientes, recebimento de mensagens e \
               realização de campanhas."),
            P("Ao interagir com a DrinkUp em redes sociais, como Instagram, Facebook, TikTok, \
               WhatsApp ou outras plataformas, alguns dados também poderão ser tratados pelas \
               próprias plataformas, de acordo com suas respectivas políticas de privacidade e \
               termos de uso."),
            P("A DrinkUp não controla diretamente o funcionamento, as regras de privacidade, os \
               cookies ou as tecnologias utilizadas por plataformas de terceiros. Recomendamos \
               que o usuário consulte as políticas de privacidade dessas plataformas para \
               entender como seus dados são tratados por elas."),
        ],
    ),
    (
        "12. Alterações desta Política",
        &[
            P("A DrinkUp poderá atualizar esta Política de Privacidade a qualquer momento para \
               refletir mudanças em seus processos, sistemas, serviços, canais de atendimento, \
               exigências legais ou práticas de segurança."),
            P("A versão mais recente desta Política poderá ser disponibilizada nos canais \
               oficiais da DrinkUp."),
            P("Alterações relevantes poderão ser comunicadas por meio do site, WhatsApp, redes \
               sociais, e-mail ou outro canal adequado."),
        ],
    ),
];

/// Bloco de contato (seção 13), com rótulos.
const CONTATO: &[(&str, &str)] = &[
    ("Empresa", "DrinkUp"),
    ("CNPJ", "45.185.460/0001-70"),
    ("Endereço", "Av. Pedro Taques, 5217 - Lot. Sumaré, Maringá - PR, CEP 87035-591"),
    ("Horário de atendimento", "Segunda a Sexta-feira, das 08h às 17h"),
    ("Canais de atendimento", "WhatsApp e redes sociais oficiais da DrinkUp"),
];

/// Renderiza os blocos de uma seção, agrupando itens de lista (`Li`) em um `<ul>`.
fn blocos_view(blocos: &'static [Bloco]) -> AnyView {
    let mut saida: Vec<AnyView> = Vec::new();
    let mut lista: Vec<&'static str> = Vec::new();
    for b in blocos {
        match b {
            Li(t) => lista.push(t),
            P(t) => {
                fechar_lista(&mut lista, &mut saida);
                saida.push(view! { <p class="legal__clausula">{*t}</p> }.into_any());
            }
            Item(rotulo, texto) => {
                fechar_lista(&mut lista, &mut saida);
                saida.push(
                    view! {
                        <p class="legal__item">
                            <strong>{format!("{rotulo}: ")}</strong>
                            {*texto}
                        </p>
                    }
                    .into_any(),
                );
            }
        }
    }
    fechar_lista(&mut lista, &mut saida);
    saida.into_any()
}

/// Esvazia os itens acumulados num `<ul>`, preservando a ordem.
fn fechar_lista(lista: &mut Vec<&'static str>, saida: &mut Vec<AnyView>) {
    if lista.is_empty() {
        return;
    }
    let itens = std::mem::take(lista);
    saida.push(
        view! {
            <ul class="legal__lista">
                {itens.into_iter().map(|t| view! { <li>{t}</li> }).collect_view()}
            </ul>
        }
        .into_any(),
    );
}

/// Página estática de Política de Privacidade.
#[component]
pub fn PrivacidadePage() -> impl IntoView {
    view! {
        <Seo
            titulo="Política de Privacidade"
            descricao="Política de Privacidade da DRINK UP: como coletamos, usamos, \
            armazenamos, compartilhamos e protegemos seus dados pessoais, em conformidade \
            com a LGPD (Lei nº 13.709/2018)."
            caminho="/politica-de-privacidade"
        />

        <section class="legal-hero">
            <div class="container">
                <h1 class="legal-hero__title">"Política de Privacidade"</h1>
                <p class="legal-hero__sub">"Última atualização: junho de 2026"</p>
            </div>
        </section>

        <article class="legal container">
            {PREAMBULO
                .iter()
                .map(|p| view! { <p class="legal__intro">{*p}</p> })
                .collect_view()}

            {SECOES
                .iter()
                .map(|(titulo, blocos)| {
                    view! {
                        <section class="legal__sec">
                            <h2 class="legal__sec-title">{*titulo}</h2>
                            {blocos_view(blocos)}
                        </section>
                    }
                })
                .collect_view()}

            <section class="legal__sec">
                <h2 class="legal__sec-title">"13. Contato"</h2>
                <p class="legal__clausula">
                    "Em caso de dúvidas sobre esta Política de Privacidade, tratamento de dados \
                    pessoais, orçamento, atendimento, resolução de solicitações ou exercício de \
                    direitos previstos na LGPD, entre em contato com a DrinkUp por meio dos nossos \
                    canais oficiais."
                </p>
                <dl class="legal__contato">
                    {CONTATO
                        .iter()
                        .map(|(rotulo, valor)| {
                            view! {
                                <div class="legal__contato-item">
                                    <dt>{*rotulo}</dt>
                                    <dd>{*valor}</dd>
                                </div>
                            }
                        })
                        .collect_view()}
                </dl>
                <p class="legal__clausula">
                    "As solicitações relacionadas a orçamento, pedidos, atendimento, resolução de \
                    problemas e exercício de direitos sobre dados pessoais serão recebidos pelos \
                    canais oficiais da DrinkUp e respondidas conforme a natureza da solicitação e \
                    os prazos aplicáveis."
                </p>
            </section>
        </article>
    }
}
