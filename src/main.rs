// Camada de apresentacao: janela Slint que funciona como um "console" grafico.
// Tudo que normalmente iria para o stdout do CLI sera jogado nesta caixa de texto.

slint::slint! {
    import { TextEdit, Button, VerticalBox } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "Ramsey-Putnam";
        preferred-width: 800px;
        preferred-height: 600px;

        // Texto preenchido pelo Rust; e o "stdout" da nossa janela.
        in property <string> console-text;

        // Evento disparado pelo botao; o Rust decide o que fazer (ver on_rodar).
        callback rodar();

        VerticalBox {
            Button {
                text: "Calcular arestas de K6";
                clicked => { root.rodar(); }
            }
            TextEdit {
                read-only: true;
                text: root.console-text;
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    ui.set_console_text(
        "Ramsey-Putnam — console grafico\n\
         --------------------------------\n\
         Clique no botao para calcular.\n\
         \n"
            .into(),
    );

    // Referencia "fraca" para usar a janela dentro da closure sem mover/ciclar.
    let ui_weak = ui.as_weak();
    ui.on_rodar(move || {
        let ui = ui_weak.upgrade().unwrap();

        // K_n completo: numero de arestas = n*(n-1)/2. Para K_6 -> 15.
        let n: u32 = 6;
        let arestas = n * (n - 1) / 2;

        // Acumula a saida, como um console que vai recebendo linhas.
        let atual = ui.get_console_text();
        ui.set_console_text(format!("{atual}K_{n} tem {arestas} arestas.\n").into());
    });

    ui.run()
}
