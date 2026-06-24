// Controle da aplicacao (a "cola" entre a UI e o dominio).
// include_modules!() traz para ca o codigo gerado a partir de ui/app.slint.
slint::include_modules!();

use slint::ComponentHandle;

// Parametros do calculo. Como o Rust NAO tem argumento default (ao contrario do
// PHP), o padrao idiomatico e juntar tudo num struct e dar a ele um Default.
// Depois da para sobrescrever so o que quiser: Parametros { n: 10, ..Default::default() }
#[derive(Clone, Copy)]
struct Parametros {
    n: u32, // tamanho do grafo completo K_n
    s: u32, // primeiro alvo de Ramsey, R(s, t)
    t: u32, // segundo alvo de Ramsey, R(s, t)
    k: u32, // sobra/extra, por enquanto so para variar
}

impl Default for Parametros {
    fn default() -> Self {
        Self { n: 6, s: 3, t: 3, k: 0 }
    }
}

// Tudo que acontece ANTES de apertar o botao: cria a janela, prepara o estado
// (no futuro, pedir parametros aqui) e liga o botao a calcular().
pub fn run() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    ui.set_console_text(
        "Ramsey-Putnam — console grafico\n\
         --------------------------------\n\
         Clique no botao para comecar o calculo.\n\
         \n"
            .into(),
    );

    // O botao apenas delega: ao disparar 'rodar', chamamos calcular() com os
    // valores default (por enquanto o botao so manda comecar).
    let ui_weak = ui.as_weak();
    ui.on_rodar(move || {
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };
        calcular(&ui, Parametros::default());
    });

    ui.run()
}

// Acao principal, acionada pelo botao. Aqui vai ficar o grosso do codigo.
// Por enquanto so devolve um valor dumb a partir dos parametros recebidos.
fn calcular(ui: &MainWindow, p: Parametros) {
    let arestas = p.n * (p.n - 1) / 2;

    let atual = ui.get_console_text();
    ui.set_console_text(
        format!(
            "{atual}calcular(n={}, s={}, t={}, k={}) -> K_{} tem {} arestas.\n",
            p.n, p.s, p.t, p.k, p.n, arestas,
        )
        .into(),
    );
}
