use leptos::prelude::*;

use crate::components::Seo;

/// Texto introdutório (antes das seções numeradas).
const PREAMBULO: &[&str] = &[
    "A Drink Up Copos e Brindes, pessoa jurídica de direito privado, inscrita no CNPJ sob o \
     nº 28.144.674/0001-89, com sede na Av. Pedro Taques, 5217 - Lot. Sumaré, Maringá - PR, \
     CEP 87035-591, oferece produtos personalizados e não personalizados, incluindo copos, \
     taças, canecas, baldes, garrafas, tampas e demais itens de seu portfólio.",
    "Estes Termos e Condições regulam a solicitação de orçamentos, compra de produtos, \
     contratação de serviços de personalização, envio de artes, aprovação de pedidos, \
     pagamentos, entregas, trocas, devoluções, reclamações e demais relações comerciais \
     entre a DrinkUp e seus clientes.",
    "Ao solicitar orçamento, aprovar uma proposta, realizar um pedido, enviar uma arte, \
     efetuar pagamento, retirar ou receber produtos, o cliente declara ter lido, \
     compreendido e aceitado estes Termos e Condições.",
    "A aceitação destes Termos é condição essencial para a contratação dos produtos e \
     serviços da DrinkUp.",
];

/// Seções numeradas: (título, cláusulas).
type Secao = (&'static str, &'static [&'static str]);

const SECOES: &[Secao] = &[
    (
        "1. Introdução",
        &[
            "1.1. Estes Termos e Condições se aplicam aos produtos e serviços oferecidos pela \
             DrinkUp por meio de seus canais oficiais de atendimento, incluindo WhatsApp, redes \
             sociais, site, telefone, e-mail, equipe comercial e demais canais utilizados pela \
             empresa.",
            "1.2. A DrinkUp poderá atender pessoas físicas, pessoas jurídicas, revendedores, \
             brindeiros, empresas, organizadores de eventos, instituições e demais clientes \
             interessados em adquirir produtos personalizados ou sem personalização.",
            "1.3. O cliente declara que todas as informações fornecidas à DrinkUp são \
             verdadeiras, completas, atualizadas e de sua responsabilidade.",
            "1.4. A DrinkUp poderá recusar, suspender ou cancelar pedidos quando houver \
             informações incorretas, indícios de fraude, descumprimento destes Termos, \
             impossibilidade técnica de produção, falta de pagamento, conteúdo inadequado na arte \
             ou qualquer situação que possa prejudicar a empresa, terceiros ou a legalidade da \
             operação.",
            "1.5. Estes Termos poderão ser atualizados a qualquer momento, conforme alterações \
             nos processos internos, produtos, serviços, legislação aplicável ou critérios \
             comerciais da DrinkUp.",
            "1.6. Cada contratação estará sujeita aos Termos e Condições vigentes no momento da \
             aprovação do orçamento ou fechamento do pedido.",
        ],
    ),
    (
        "2. Atendimento, cadastro e informações do cliente",
        &[
            "2.1. O atendimento da DrinkUp é realizado, preferencialmente, por WhatsApp e redes \
             sociais oficiais, de segunda a sexta-feira, das 08h às 17h, exceto feriados ou \
             períodos previamente comunicados.",
            "2.2. Para solicitação de orçamento, compra, faturamento, produção, entrega ou \
             retirada, o cliente poderá precisar informar dados como nome completo, telefone, \
             e-mail, CPF, CNPJ, razão social, inscrição estadual, endereço de entrega, endereço \
             de cobrança e demais informações necessárias para atendimento.",
            "2.3. O cliente é responsável pela veracidade dos dados fornecidos. Informações \
             incorretas, incompletas ou desatualizadas podem causar atraso, erro de emissão \
             fiscal, falha na entrega ou impossibilidade de produção.",
            "2.4. Em compras realizadas por pessoa jurídica, o solicitante declara possuir \
             autorização para negociar, aprovar pedidos, enviar artes, aceitar propostas e \
             contratar produtos ou serviços em nome da empresa representada.",
            "2.5. Caso o cliente solicite alteração de dados após a aprovação do pedido, a \
             DrinkUp analisará a viabilidade da alteração. Mudanças podem não ser possíveis caso \
             o pedido já esteja em produção, faturado, separado, embalado ou enviado.",
            "2.6. As informações pessoais fornecidas pelo cliente serão tratadas conforme a \
             Política de Privacidade da DrinkUp e a legislação aplicável de proteção de dados.",
        ],
    ),
    (
        "3. Produtos e serviços oferecidos",
        &[
            "3.1. A DrinkUp comercializa produtos personalizados e sem personalização, podendo \
             atuar em pedidos de varejo, atacado, eventos, ações promocionais, revenda, brindes \
             corporativos e demais finalidades compatíveis com seu portfólio.",
            "3.2. Os produtos podem incluir, entre outros, copos, taças, canecas, baldes, \
             garrafas, tampas, tirantes, acessórios e itens relacionados.",
            "3.3. A personalização poderá variar conforme o produto, material, cor, quantidade, \
             técnica disponível, área de impressão e viabilidade técnica.",
            "3.4. Entre as técnicas de personalização utilizadas pela DrinkUp podem estar \
             serigrafia, IML/label, transfer, sublimação ou outras técnicas disponibilizadas \
             pela empresa.",
            "3.5. Nem todos os produtos aceitam todos os tipos de personalização. A \
             disponibilidade será informada pela equipe comercial conforme o item solicitado.",
            "3.6. A DrinkUp poderá alterar, incluir ou descontinuar produtos, cores, modelos, \
             acabamentos, acessórios, técnicas de personalização e condições comerciais sem aviso \
             prévio, preservando os pedidos já aprovados conforme as condições acordadas.",
        ],
    ),
    (
        "4. Orçamentos e propostas comerciais",
        &[
            "4.1. Os orçamentos solicitados à DrinkUp serão elaborados com base nas informações \
             fornecidas pelo cliente, como produto, quantidade, cor, tipo de personalização, \
             quantidade de cores, prazo desejado, endereço de entrega e demais características do \
             pedido.",
            "4.2. O orçamento poderá ser enviado por WhatsApp, redes sociais, e-mail, sistema \
             interno ou outro canal oficial de atendimento.",
            "4.3. O valor informado no orçamento poderá variar caso o cliente altere quantidade, \
             produto, cor, tipo de personalização, arte, prazo, endereço de entrega, modalidade \
             de envio ou qualquer outra condição do pedido.",
            "4.4. O orçamento terá validade conforme informado pela equipe comercial. Na ausência \
             de prazo específico, a DrinkUp poderá revisar valores e condições antes da \
             confirmação do pedido.",
            "4.5. A aprovação do orçamento pelo cliente representa sua concordância com os \
             valores, produtos, quantidades, prazos, condições comerciais e demais informações \
             descritas na proposta.",
            "4.6. A DrinkUp poderá cancelar orçamentos sem movimentação, sem retorno do cliente \
             ou sem pagamento dentro do prazo informado no atendimento.",
        ],
    ),
    (
        "5. Pedido",
        &[
            "5.1. O pedido será considerado iniciado somente após a confirmação das informações \
             necessárias, aprovação do orçamento, envio ou aprovação da arte quando aplicável e \
             confirmação do pagamento ou liberação comercial pela DrinkUp.",
            "5.2. A confirmação do pedido poderá exigir informações como dados cadastrais, dados \
             fiscais, endereço de entrega, produto, quantidade, cor, tipo de personalização, arte \
             aprovada e forma de pagamento.",
            "5.3. O cliente deverá conferir atentamente todos os dados do pedido antes da \
             aprovação, incluindo produto, quantidade, cor, medidas, personalização, nomes, \
             datas, frases, logotipos, ortografia, cores, layout, prazos e endereço de entrega.",
            "5.4. Após a aprovação do pedido e da arte, alterações podem não ser possíveis. Caso \
             sejam solicitadas mudanças após aprovação, a DrinkUp avaliará a viabilidade e poderá \
             haver ajuste de prazo, custo adicional ou impossibilidade de alteração.",
            "5.5. Pedidos personalizados são produzidos sob demanda e de acordo com as \
             informações aprovadas pelo cliente.",
            "5.6. A DrinkUp não se responsabiliza por erros aprovados pelo cliente em nomes, \
             frases, datas, telefones, logotipos, informações de evento, cores, posicionamento, \
             ortografia ou demais elementos da arte.",
            "5.7. Caso o pagamento não seja realizado, seja recusado, cancelado ou não \
             identificado, o pedido poderá ser suspenso ou cancelado.",
        ],
    ),
    (
        "6. Artes, arquivos e personalização",
        &[
            "6.1. A arte enviada ou aprovada pelo cliente é de sua inteira responsabilidade.",
            "6.2. O cliente deverá enviar arquivos em formato adequado para produção, respeitando \
             as orientações técnicas fornecidas pela DrinkUp, incluindo gabaritos, áreas de \
             segurança, sangria, resolução, cores, vetorização, curvas, tamanho mínimo de texto e \
             demais especificações.",
            "6.3. A DrinkUp poderá aceitar arquivos em formatos como PDF, AI, EPS, CDR, PNG, JPG \
             ou outros formatos informados pela equipe, conforme a finalidade e o tipo de \
             personalização.",
            "6.4. O envio de arquivo não garante automaticamente a viabilidade de produção. A \
             DrinkUp poderá solicitar ajustes quando identificar problemas técnicos, baixa \
             qualidade, ausência de vetor, baixa resolução, elementos fora da área de segurança \
             ou incompatibilidade com a técnica escolhida.",
            "6.5. Para personalizações em serigrafia, o cliente deverá observar que a técnica \
             exige arte adequada, preferencialmente vetorial, com traços firmes, sem gradientes \
             complexos, sem detalhes excessivamente finos e respeitando as limitações de cores e \
             registro.",
            "6.6. Para personalizações full color, label, IML ou similares, o cliente deverá \
             observar resolução adequada, modo de cor indicado, área de segurança, possíveis \
             variações de tonalidade e limitações do processo produtivo.",
            "6.7. Para personalizações por transfer, sublimação ou outras técnicas específicas, o \
             cliente deverá seguir as orientações fornecidas pela equipe DrinkUp para cada \
             produto.",
            "6.8. Textos, logotipos e elementos importantes devem permanecer dentro da área de \
             segurança indicada no gabarito. Elementos fora dessa área podem ser cortados, \
             distorcidos ou sofrer variação no resultado final.",
            "6.9. A DrinkUp poderá auxiliar na criação, ajuste ou adaptação de artes quando esse \
             serviço estiver disponível e for previamente acordado com o cliente.",
            "6.10. Ajustes de arte, vetorização, recriação, tratamento de imagem, mudança de \
             layout ou criação personalizada podem gerar custo adicional e alterar o prazo de \
             produção.",
            "6.11. A aprovação da arte pelo cliente autoriza a DrinkUp a seguir com a produção.",
            "6.12. Após a aprovação da arte, a DrinkUp não se responsabiliza por erros de \
             conferência, incluindo ortografia, nomes, datas, telefones, marcas, logos, \
             posicionamento, cores escolhidas ou qualquer informação visual aprovada.",
            "6.13. A DrinkUp poderá recusar a produção de artes que contenham conteúdo ofensivo, \
             discriminatório, pornográfico, ilegal, violento, difamatório, que infrinja direitos \
             de terceiros, que viole marcas, direitos autorais, imagem de pessoas ou qualquer \
             norma aplicável.",
            "6.14. O cliente declara que possui autorização para utilizar todos os elementos \
             enviados na arte, incluindo logotipos, marcas, personagens, imagens, fotos, nomes, \
             brasões, símbolos, frases, ilustrações e qualquer conteúdo protegido por propriedade \
             intelectual ou direito de imagem.",
            "6.15. Caso a arte contenha imagem, nome ou dados de terceiros, incluindo crianças, \
             adolescentes, colaboradores, convidados, formandos, atletas, clientes, instituições \
             ou empresas, o cliente declara possuir as autorizações necessárias para uso e \
             reprodução.",
            "6.16. O cliente responderá integralmente por reclamações, perdas, danos, \
             notificações, indenizações, processos ou qualquer consequência decorrente do uso \
             indevido de imagens, marcas, dados, nomes, frases, personagens ou conteúdos enviados \
             à DrinkUp.",
            "6.17. A DrinkUp poderá manter arquivos e artes em seus registros pelo período \
             necessário para produção, conferência, reimpressão, histórico do pedido, atendimento \
             pós-venda ou recompra, observadas as regras da Política de Privacidade.",
        ],
    ),
    (
        "7. Variações de produção, cores e acabamento",
        &[
            "7.1. O cliente está ciente de que podem ocorrer pequenas variações entre a prévia \
             digital e o produto final.",
            "7.2. As cores visualizadas em telas de computador, celular ou tablet podem variar em \
             relação ao produto físico, devido a diferenças de brilho, calibração de tela, \
             iluminação, material do produto, cor da peça, tinta, processo de impressão e \
             acabamento.",
            "7.3. Produtos personalizados podem apresentar pequenas variações de alinhamento, \
             registro, tonalidade, encaixe, corte, acabamento ou posicionamento, desde que dentro \
             dos limites técnicos do processo produtivo.",
            "7.4. Em produtos com superfície curva, cônica, texturizada, transparente, colorida, \
             metalizada ou com acabamento especial, a arte pode sofrer adaptação visual, leve \
             distorção ou alteração de percepção.",
            "7.5. Cores claras impressas sobre produtos escuros, coloridos ou transparentes podem \
             apresentar variação de intensidade.",
            "7.6. Pequenas diferenças entre lotes de produtos, cores de matéria-prima, \
             acabamentos ou acessórios não serão consideradas defeito quando estiverem dentro dos \
             padrões normais de fabricação.",
            "7.7. Imagens 3D, mockups, simulações digitais, amostras virtuais ou prévias enviadas \
             durante o atendimento são meramente ilustrativas e têm como objetivo demonstrar uma \
             aproximação do resultado final.",
        ],
    ),
    (
        "8. Pagamento",
        &[
            "8.1. O pagamento do pedido poderá ser realizado pelos meios disponibilizados pela \
             DrinkUp no momento da compra, como PIX, boleto, cartão, transferência bancária ou \
             outra forma informada pela equipe comercial.",
            "8.2. O pedido somente seguirá para produção após a confirmação do pagamento ou \
             liberação comercial pela DrinkUp, quando aplicável.",
            "8.3. Pagamentos por PIX, boleto ou cartão estarão sujeitos aos prazos de compensação \
             e validação das instituições financeiras ou intermediadores de pagamento.",
            "8.4. Caso o pagamento não seja realizado dentro do prazo informado, o orçamento ou \
             pedido poderá ser cancelado.",
            "8.5. Em caso de pagamento posterior, faturado ou condição especial concedida ao \
             cliente, a aprovação dependerá de análise e liberação da DrinkUp.",
            "8.6. Atrasos em pagamentos faturados ou posteriores poderão gerar cobrança de multa, \
             juros, correção, suspensão de novos pedidos e adoção das medidas cabíveis para \
             recebimento dos valores devidos.",
            "8.7. A DrinkUp poderá solicitar comprovantes, documentos ou informações adicionais \
             para identificação e conciliação de pagamentos.",
        ],
    ),
    (
        "9. Nota fiscal",
        &[
            "9.1. A nota fiscal será emitida conforme os dados fornecidos pelo cliente.",
            "9.2. O cliente é responsável por conferir previamente os dados fiscais, incluindo \
             nome, razão social, CPF, CNPJ, inscrição estadual, endereço e demais informações \
             necessárias.",
            "9.3. Após a emissão da nota fiscal, alterações poderão não ser possíveis, \
             especialmente quando houver limitação legal, fiscal, contábil ou sistêmica.",
            "9.4. Caso o cliente forneça dados fiscais incorretos, eventuais atrasos, custos, \
             cancelamentos ou impossibilidades de correção serão de sua responsabilidade.",
        ],
    ),
    (
        "10. Entrega, retirada e prazo",
        &[
            "10.1. O prazo de produção e entrega será informado pela equipe DrinkUp conforme \
             produto, quantidade, personalização, aprovação de arte, confirmação de pagamento, \
             disponibilidade de estoque, modalidade de entrega e localidade do cliente.",
            "10.2. O prazo de produção começa a contar somente após a confirmação do pagamento ou \
             liberação comercial, aprovação da arte, recebimento de todas as informações \
             necessárias e validação do pedido pela DrinkUp.",
            "10.3. Caso o cliente solicite alterações após a aprovação, o prazo poderá ser \
             reiniciado ou ajustado conforme a etapa do pedido.",
            "10.4. A entrega poderá ser realizada por transportadora, Correios, motoboy, retirada \
             no local ou outro meio disponível no momento da contratação.",
            "10.5. Quando a entrega for realizada por transportadora, Correios ou terceiro \
             contratado, o prazo poderá variar conforme a empresa responsável pelo transporte.",
            "10.6. A DrinkUp não se responsabiliza por atrasos decorrentes de fatores externos, \
             como endereço incorreto, ausência de responsável no local, recusa de recebimento, \
             restrições de entrega, problemas climáticos, greves, falhas operacionais da \
             transportadora, força maior ou caso fortuito.",
            "10.7. A entrega poderá ser realizada a qualquer pessoa que esteja no endereço \
             informado pelo cliente e aceite o recebimento.",
            "10.8. O cliente é responsável por informar corretamente o endereço de entrega, \
             incluindo rua, número, bairro, cidade, estado, CEP, complemento, ponto de referência \
             e telefone de contato.",
            "10.9. Após o pedido entrar em produção, faturamento, separação, embalagem ou envio, \
             a alteração de endereço poderá não ser possível.",
            "10.10. Em caso de falha de entrega por erro de endereço, ausência do destinatário ou \
             outro motivo não causado pela DrinkUp, custos de reenvio, nova tentativa de entrega \
             ou retorno poderão ser cobrados do cliente.",
            "10.11. Nos casos de retirada no endereço da DrinkUp, o cliente será informado quando \
             o pedido estiver disponível.",
            "10.12. A retirada deverá ocorrer durante o horário de atendimento da DrinkUp, de \
             segunda a sexta-feira, das 08h às 17h, salvo comunicação diferente feita pela \
             empresa.",
            "10.13. Produtos não retirados dentro do prazo informado pela equipe poderão ficar \
             sujeitos a armazenamento temporário, nova combinação de retirada ou outras medidas \
             cabíveis.",
        ],
    ),
    (
        "11. Conferência no recebimento",
        &[
            "11.1. O cliente deverá conferir o pedido no momento do recebimento ou retirada.",
            "11.2. A conferência deve incluir quantidade de volumes, integridade da embalagem, \
             sinais de avaria, produto recebido, quantidade, modelo, cor e personalização.",
            "11.3. Caso haja indício de avaria no transporte, violação de embalagem ou dano \
             visível, o cliente deverá registrar fotos e comunicar a DrinkUp pelos canais \
             oficiais o mais rápido possível.",
            "11.4. A ausência de comunicação em prazo razoável poderá dificultar ou inviabilizar \
             a análise junto à transportadora ou a adoção de medidas corretivas.",
        ],
    ),
    (
        "12. Reclamações, trocas, devoluções e reimpressões",
        &[
            "12.1. Reclamações sobre defeitos, avarias, divergências ou problemas no pedido \
             deverão ser feitas por meio dos canais oficiais de atendimento da DrinkUp.",
            "12.2. O cliente deverá apresentar informações que permitam a análise da solicitação, \
             como número do pedido, nota fiscal, fotos, vídeos, descrição do problema, quantidade \
             afetada e imagens da embalagem, quando aplicável.",
            "12.3. O prazo para reclamações relacionadas a defeitos aparentes, avarias ou \
             divergências deverá observar a legislação aplicável e as orientações informadas pela \
             equipe de atendimento.",
            "12.4. Para análise de eventual defeito, a DrinkUp poderá solicitar fotos que mostrem \
             o problema, fotos da quantidade afetada em relação ao pedido total, vídeos, \
             devolução de amostras ou devolução dos produtos supostamente defeituosos.",
            "12.5. Os produtos enviados para análise deverão estar, sempre que possível, sem uso, \
             com embalagem original ou acondicionados de forma adequada.",
            "12.6. A DrinkUp analisará a solicitação e, caso seja constatada falha de produção, \
             poderá oferecer uma das seguintes soluções, conforme o caso: correção, reimpressão, \
             substituição, crédito, abatimento ou estorno.",
            "12.7. Não será considerada falha de produção quando o problema decorrer de arte \
             aprovada com erro, arquivo em baixa qualidade, informações incorretas enviadas pelo \
             cliente, uso inadequado do produto, armazenamento incorreto, danos causados por \
             terceiros, transporte, desgaste natural ou variações técnicas normais do processo \
             produtivo.",
            "12.8. Não haverá reimpressão gratuita quando o erro estiver relacionado a ortografia, \
             nomes, datas, telefones, logotipos, informações do evento, cores escolhidas, \
             posicionamento, baixa resolução, elementos fora da área de segurança ou qualquer \
             detalhe aprovado pelo cliente.",
            "12.9. Em produtos personalizados, o direito de arrependimento poderá ser analisado \
             conforme a legislação aplicável, considerando que se trata de item produzido sob \
             encomenda e com características definidas pelo cliente.",
            "12.10. Quando houver estorno, o valor será devolvido preferencialmente pelo mesmo \
             meio de pagamento utilizado na compra, observados os prazos das instituições \
             financeiras, operadoras de cartão, bancos ou intermediadores.",
            "12.11. Em caso de pagamento por PIX, boleto ou transferência, a DrinkUp poderá \
             solicitar dados bancários de titularidade compatível com o comprador, responsável \
             financeiro ou empresa indicada na nota fiscal.",
        ],
    ),
    (
        "13. Cancelamento de pedido",
        &[
            "13.1. O cliente poderá solicitar cancelamento do pedido pelos canais oficiais da \
             DrinkUp.",
            "13.2. O cancelamento será analisado conforme a etapa em que o pedido se encontra.",
            "13.3. Caso o pedido ainda não tenha entrado em produção, personalização, separação, \
             faturamento ou envio, o cancelamento poderá ser realizado conforme análise da \
             DrinkUp.",
            "13.4. Caso o pedido já tenha entrado em produção, personalização, corte, impressão, \
             separação, faturamento, embalagem ou envio, o cancelamento poderá não ser possível, \
             especialmente em produtos personalizados.",
            "13.5. Custos já incorridos com criação de arte, ajustes, matéria-prima, produção, \
             personalização, prova, embalagem, frete ou taxas poderão ser descontados de eventual \
             valor a devolver, quando aplicável.",
        ],
    ),
    (
        "14. Provas, amostras e aprovações especiais",
        &[
            "14.1. Quando solicitado pelo cliente e disponível pela DrinkUp, poderá ser produzida \
             prova, amostra ou unidade teste para validação de produto, cor, impressão ou \
             acabamento.",
            "14.2. A produção de prova ou amostra poderá gerar custo adicional e alterar o prazo \
             final do pedido.",
            "14.3. O prazo de produção do pedido principal poderá passar a contar após a \
             aprovação da prova ou amostra pelo cliente.",
            "14.4. A prova ou amostra tem como finalidade demonstrar uma referência do resultado \
             final, podendo ainda haver pequenas variações técnicas na produção em escala.",
        ],
    ),
    (
        "15. Pedidos com conteúdo de propaganda eleitoral",
        &[
            "15.1. Quando o pedido envolver conteúdo de propaganda eleitoral, o cliente declara \
             estar ciente de que deverá observar integralmente a legislação eleitoral vigente.",
            "15.2. O cliente é responsável por fornecer corretamente todos os dados exigidos para \
             emissão de nota fiscal, identificação do contratante, candidato, partido, CNPJ, CPF, \
             tiragem, quantidade e demais informações obrigatórias.",
            "15.3. Quando aplicável, os materiais deverão conter as informações exigidas pela \
             legislação eleitoral, incluindo identificação do responsável pela contratação, \
             empresa contratada, tiragem e demais elementos obrigatórios.",
            "15.4. A DrinkUp poderá solicitar informações complementares, recusar produção, \
             suspender pedido ou exigir ajustes caso identifique ausência de dados obrigatórios \
             ou possível descumprimento da legislação eleitoral.",
            "15.5. A responsabilidade pelo conteúdo eleitoral, regularidade da campanha, \
             autorização de uso de imagem, informações do candidato, partido, coligação, \
             federação ou responsável pela contratação será integralmente do cliente.",
        ],
    ),
    (
        "16. Responsabilidades do cliente",
        &[
            "16.1. O cliente é responsável por fornecer informações verdadeiras, completas e \
             atualizadas.",
            "16.2. O cliente é responsável por conferir orçamento, pedido, arte, dados fiscais, \
             endereço de entrega, prazos e condições comerciais antes da aprovação.",
            "16.3. O cliente é responsável por garantir que possui autorização para utilizar \
             imagens, marcas, logotipos, nomes, frases, fotografias, personagens, símbolos, \
             brasões, instituições e demais conteúdos enviados à DrinkUp.",
            "16.4. O cliente é responsável por observar a legislação aplicável ao uso dos \
             produtos adquiridos, especialmente quando forem utilizados em eventos, campanhas, \
             ações promocionais, instituições, empresas, turmas, formaturas, eleições ou revenda.",
            "16.5. O cliente se compromete a não utilizar os produtos ou serviços da DrinkUp para \
             fins ilícitos, fraudulentos, ofensivos, discriminatórios ou que possam causar danos \
             a terceiros.",
            "16.6. Caso a DrinkUp sofra reclamação, notificação, processo, multa, prejuízo ou \
             qualquer dano em decorrência de conteúdo enviado ou aprovado pelo cliente, este \
             deverá assumir a responsabilidade e indenizar a DrinkUp pelos prejuízos causados.",
        ],
    ),
    (
        "17. Responsabilidades da DrinkUp",
        &[
            "17.1. A responsabilidade da DrinkUp limita-se à produção e entrega dos produtos e \
             serviços contratados, conforme condições aprovadas, viabilidade técnica e \
             informações fornecidas pelo cliente.",
            "17.2. A DrinkUp não se responsabiliza por erros decorrentes de informações \
             incorretas fornecidas pelo cliente, aprovação indevida de arte, uso inadequado dos \
             produtos, armazenamento incorreto, danos causados por terceiros, transporte ou \
             fatores externos fora de seu controle.",
            "17.3. A DrinkUp não garante que seus canais digitais, site, redes sociais ou \
             sistemas estejam sempre disponíveis, livres de falhas, interrupções, vírus, ataques, \
             instabilidades ou erros causados por terceiros.",
            "17.4. A DrinkUp poderá suspender temporariamente canais, serviços, site ou sistemas \
             para manutenção, atualização, ajustes técnicos ou situações fora de seu controle.",
        ],
    ),
    (
        "18. Direito de imagem e propriedade intelectual",
        &[
            "18.1. A marca DrinkUp, seu nome, logotipo, identidade visual, materiais \
             institucionais, fotos, vídeos, textos, catálogos, artes, imagens comerciais, \
             conteúdos de site, redes sociais e demais materiais pertencem à DrinkUp ou são \
             utilizados mediante autorização.",
            "18.2. O cliente não poderá copiar, reproduzir, modificar, distribuir, comercializar \
             ou utilizar materiais da DrinkUp sem autorização prévia.",
            "18.3. O cliente garante que todo conteúdo enviado para personalização é de sua \
             propriedade, está licenciado ou possui autorização de uso.",
            "18.4. O cliente garante que a arte enviada não viola marcas, direitos autorais, \
             direitos de imagem, propriedade intelectual, dados pessoais ou qualquer direito de \
             terceiros.",
            "18.5. A DrinkUp poderá registrar fotos e vídeos dos produtos produzidos, do processo \
             produtivo e do resultado final para fins de histórico interno, controle de \
             qualidade, portfólio, divulgação, publicidade, redes sociais, campanhas e materiais \
             institucionais.",
            "18.6. Caso o cliente não autorize a divulgação de determinado produto personalizado, \
             deverá informar a DrinkUp por escrito antes da produção ou no momento da contratação.",
            "18.7. A autorização de uso de imagens dos produtos pela DrinkUp não implica \
             divulgação de dados sensíveis, informações confidenciais ou estratégias comerciais \
             do cliente, salvo quando tais informações estiverem visíveis na própria arte \
             aprovada para produção.",
        ],
    ),
    (
        "19. Privacidade e proteção de dados",
        &[
            "19.1. A DrinkUp tratará os dados pessoais dos clientes conforme sua Política de \
             Privacidade e a legislação aplicável.",
            "19.2. Os dados poderão ser utilizados para atendimento, orçamento, pedido, emissão \
             de nota fiscal, produção, personalização, entrega, pós-venda, resolução de \
             solicitações, cumprimento de obrigações legais e comunicação com o cliente.",
            "19.3. Ao enviar dados pessoais, imagens, nomes, fotos ou informações de terceiros \
             para personalização de produtos, o cliente declara possuir autorização para esse \
             uso.",
            "19.4. Solicitações relacionadas a dados pessoais poderão ser realizadas pelos canais \
             oficiais de atendimento da DrinkUp.",
        ],
    ),
    (
        "20. Campanhas promocionais e condições especiais",
        &[
            "20.1. Campanhas promocionais, descontos, brindes, condições especiais, programas de \
             venda, ações sazonais ou datas comemorativas poderão ter regras próprias.",
            "20.2. As regras específicas de cada campanha serão informadas nos canais oficiais da \
             DrinkUp ou no atendimento comercial.",
            "20.3. Promoções poderão ter prazo de validade, limite de estoque, quantidade mínima, \
             produtos selecionados, condições de pagamento específicas e outras restrições.",
            "20.4. A DrinkUp poderá encerrar, alterar ou prorrogar campanhas promocionais \
             conforme critérios comerciais, respeitando pedidos já aprovados dentro das condições \
             divulgadas.",
        ],
    ),
    (
        "21. Disposições gerais",
        &[
            "21.1. Estes Termos e Condições constituem o acordo entre a DrinkUp e o cliente em \
             relação à compra de produtos, contratação de serviços, envio de artes, \
             personalização, entrega e atendimento.",
            "21.2. Caso qualquer cláusula destes Termos seja considerada inválida ou inexequível, \
             as demais permanecerão válidas e aplicáveis.",
            "21.3. A eventual tolerância da DrinkUp quanto ao descumprimento de qualquer condição \
             não será considerada renúncia de direito.",
            "21.4. Estes Termos não estabelecem sociedade, vínculo empregatício, representação \
             comercial automática, parceria obrigatória ou solidariedade entre a DrinkUp e o \
             cliente.",
            "21.5. Estes Termos e Condições serão regidos pelas leis da República Federativa do \
             Brasil.",
            "21.6. Fica eleito o Foro da Comarca de Maringá, Estado do Paraná, para dirimir \
             eventuais controvérsias decorrentes destes Termos, salvo hipóteses legais de \
             competência obrigatória diversa.",
        ],
    ),
];

/// Bloco de contato (seção 22), com rótulos.
const CONTATO: &[(&str, &str)] = &[
    ("Empresa", "Dk Comercio de Copos e Brindes LTDA"),
    ("CNPJ", "28.144.674/0001-89"),
    ("Endereço", "Av. Pedro Taques, 5217 - Lot. Sumaré, Maringá - PR, CEP 87035-591"),
    ("Horário de atendimento", "Segunda a Sexta-feira, das 08h às 17h"),
    ("Canais de atendimento", "WhatsApp e redes sociais oficiais da DrinkUp"),
];

/// Página estática de Termos e Condições.
#[component]
pub fn TermosPage() -> impl IntoView {
    view! {
        <Seo
            titulo="Termos e Condições"
            descricao="Termos e Condições da DRINK UP: orçamentos, pedidos, personalização, \
            pagamentos, entregas, trocas, devoluções e demais relações comerciais."
            caminho="/termos-de-uso"
        />

        <section class="legal-hero">
            <div class="container">
                <h1 class="legal-hero__title">"Termos e Condições"</h1>
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
                .map(|(titulo, clausulas)| {
                    view! {
                        <section class="legal__sec">
                            <h2 class="legal__sec-title">{*titulo}</h2>
                            {clausulas
                                .iter()
                                .map(|c| view! { <p class="legal__clausula">{*c}</p> })
                                .collect_view()}
                        </section>
                    }
                })
                .collect_view()}

            <section class="legal__sec">
                <h2 class="legal__sec-title">"22. Contato"</h2>
                <p class="legal__clausula">
                    "Em caso de dúvidas, solicitações, orçamentos, pedidos, reclamações, trocas, \
                    devoluções ou resolução de problemas, o cliente poderá entrar em contato com a \
                    DrinkUp por meio de seus canais oficiais."
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
            </section>
        </article>
    }
}
