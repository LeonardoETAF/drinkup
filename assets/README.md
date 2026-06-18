# Materiais de design (fonte)

Coloque aqui os arquivos **originais** de design. Esta pasta é **referência/fonte** —
não é servida pelo site. As imagens que o site realmente usa são otimizadas e
copiadas para `public/img/` nas Fases 1 e 9.

## Onde colocar cada coisa

| Pasta | O que vai aqui |
|-------|----------------|
| `assets/logos/` | Logos do DRINKUP (preferência: SVG; PNG em alta como alternativa). Variações: principal, monocromática, símbolo. |
| `assets/icons/` | Ícones da interface (SVG de preferência). Ex.: menu, busca, filtro, redes sociais, ícones do painel. |
| `assets/backgrounds/` | Imagens de fundo / banners / texturas (alta resolução). |
| `assets/design/` | **Mockups do site e do painel** (Figma export, PNG/JPG/PDF). É a referência visual que vou seguir nas telas. |

## Dicas
- **SVG** sempre que possível (logos e ícones): escala sem perda e arquivos menores.
- Para os mockups, nomeie de forma clara: `site-home.png`, `site-produto.png`,
  `painel-dashboard.png`, `painel-produtos.png`, etc.
- Se tiver o guia de estilo (cores em HEX, fontes, espaçamentos), inclua em
  `assets/design/` — uso direto para montar os tokens em `style/tokens.css`.
