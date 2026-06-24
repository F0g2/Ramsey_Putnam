// Controle da aplicacao (a "cola" entre a UI e o dominio).
// include_modules!() traz para ca o codigo gerado a partir de ui/app.slint.
slint::include_modules!();

use slint::ComponentHandle;

// Tudo que acontece ANTES de apertar o botao: cria a janela, prepara o estado
// (no futuro, pedir parametros aqui) e liga o botao a calcular().
pub fn run() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    ui.set_console_text(
        "Ramsey-Putnam — console grafico\n\
         --------------------------------\n\
         Clique no botao para calcular.\n\
         \n"
            .into(),
    );

    // O botao apenas delega: ao disparar 'rodar', chamamos calcular().
    let ui_weak = ui.as_weak();
    ui.on_rodar(move || {
        // Se a janela ja nao existir, nao ha o que fazer (sem unwrap/panic).
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };
        calcular(&ui);
    });

    ui.run()
}

// Acao principal, acionada pelo botao. Aqui vai ficar o grosso do codigo.
// Por enquanto: numero de arestas de K_n completo = n*(n-1)/2.
fn calcular(ui: &MainWindow) {
    let n: u32 = 6;
    let arestas = n * (n - 1) / 2;

    let atual = ui.get_console_text();
    ui.set_console_text(format!("{atual}K_{n} tem {arestas} arestas.\n").into());
}
