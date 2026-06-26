# Plano — GUI avançada (protótipo do Claude Design)

Reescrita incremental da interface, fiel à filosofia: controles desenhados com
**primitivos** (não std-widgets), para os 4 temas funcionarem; console próprio.
Tudo "de mentira" (sem cálculo real) até o domínio existir.

## Decisões

- **Barra de título:** nativa do SO (sem frameless). Wayland atrapalha o resto.
- **Temas:** cada tema herda de uma **base** (clara/escura) e sobrescreve só as
  cores marcantes. Liga ao `Palette.color-scheme` nativo. Temas: Terminal,
  Âmbar (CRT), Nord, Claro.
- **Dropdowns:** tentar `PopupWindow` custom (A); se enroscar, `ComboBox` (B).
- **Visualização:** só uma **área dedicada** por enquanto (sem grafo real).
- **Modularização:** `app.slint` é o "painel" (importa e compõe); detalhe pesado
  em `theme.slint`, `strings.slint` e `components/`. Estado compartilhado via
  `global` (Tema, e futuramente Estado).

## Passos

- [x] **1. Tema + modularização** — `theme.slint` (global Tema, base+override),
      `components/console.slint`, `app.slint` vira painel. Mantém o que já roda.
- [x] **2. Esqueleto da janela** — barra nativa + `topbar.slint` (abas
      Principal/Avançado + seletores idioma/tema) + divisão sidebar/main.
      `global Estado` (aba, idioma). Sidebar/main ainda placeholders.
- [x] **3. Sidebar (Principal)** — `stepper.slint` (s, t, n), dropdown Método
      (ComboBox por ora), botão Executar (primitivo). `sidebar.slint`.
- [ ] **4. Coluna direita** — console por modelo de linhas coloridas +
      `viz.slint` (área de visualização vazia).
- [ ] **5. Acabamentos** — aba Avançado (toggle, timeout, threads, verbosidade),
      dropdowns via PopupWindow, i18n PT/EN/ES, cursor piscando.

## Estrutura de arquivos (alvo)

```
ui/
├── app.slint              ← painel (compõe tudo)
├── theme.slint            ← global Tema
├── strings.slint          ← global Textos (i18n)        [Passo 5]
└── components/
    ├── topbar.slint        [Passo 2]
    ├── sidebar.slint       [Passo 3]
    ├── stepper.slint       [Passo 3]
    ├── dropdown.slint      [Passo 5]
    ├── console.slint       [feito]
    └── viz.slint           [Passo 4]
```
