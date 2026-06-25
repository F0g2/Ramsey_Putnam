# Notas técnicas — decisões e ressalvas

Registro de escolhas que merecem atenção futura (APIs instáveis, limitações de
plataforma, dívidas a revisitar). Ordem cronológica.

---

## 2026-06-25 — Fonte monospace via API instável do Slint (`fontique`)

**O quê:** o console usa **DejaVu Sans Mono** embutida no binário. O registro
da fonte é feito em `src/app.rs` (início de `run()`) através do módulo
`slint::fontique_010`, habilitado pela feature `unstable-fontique-010` em
`Cargo.toml`.

**Por quê:** o Slint **1.17** removeu as funções antigas
`register_font_from_path` / `register_font_from_memory`. O caminho oficial hoje
para registrar fonte em tempo de execução é o módulo `fontique`. Embutir a fonte
(em vez de depender da fonte do sistema) deixa o visual idêntico em
Linux/Windows/macOS.

**Ressalva (importante):** a feature `unstable-fontique-010` é, como o nome diz,
**instável** — a API pode mudar em versões futuras do Slint. Ao **atualizar a
versão do `slint`**, conferir/ajustar o bloco de registro da fonte em
`src/app.rs` (e o nome da feature em `Cargo.toml`, que pode virar
`unstable-fontique-0XX`).

**Pegadinha de ordem:** `slint::fontique_010::shared_collection()` **entra em
pânico** (`slint platform not initialized`) se chamado antes de a plataforma
existir. Por isso o registro da fonte vem **depois** de `MainWindow::new()`
(que inicializa o backend) e **antes** de `ui.run()`.

**Licença da fonte:** DejaVu Sans Mono está sob a licença Bitstream Vera
(permissiva, permite redistribuição). Texto em
`ui/assets/fonts/DejaVuSansMono-LICENSE.txt`.

---

## 2026-06-25 — Ícone da janela não aparece no Wayland (pendente)

**O quê:** definimos `Window.icon` em `ui/app.slint` apontando para
`ui/assets/Penrose-dreieck.svg.png`.

**Limitação:** no **Wayland**, o ícone definido por código é **ignorado** pelo
compositor (por design do protocolo). Quem define o ícone é um arquivo
`.desktop` instalado, casado por `app_id` — e a API estável do Slint não expõe o
`app_id` de forma limpa. No **GNOME**, a barra de título sequer exibe ícone de
app; ele aparece no dock / alt-tab, vindo do `.desktop`.

**Onde funciona:** X11, Windows e macOS usam o `Window.icon` normalmente.

**A revisitar:** quando formos **empacotar** o app, criar um `.desktop` +
instalar o ícone no tema, para aparecer no dock do Wayland. Até lá, conviver com
o ícone genérico ao rodar via `cargo run` no Wayland.
