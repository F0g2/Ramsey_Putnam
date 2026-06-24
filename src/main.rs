// Camada de apresentacao: janela Slint que funciona como um "console" grafico.
// Tudo que normalmente iria para o stdout do CLI sera jogado nesta caixa de texto.

slint::slint! {
    import { TextEdit } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "Ramsey-Putnam";
        preferred-width: 800px;
        preferred-height: 600px;

        // Texto preenchido pelo Rust; e o "stdout" da nossa janela.
        in property <string> console-text;

        TextEdit {
            width: 100%;
            height: 100%;
            read-only: true;
            text: root.console-text;
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    // Texto de exemplo: e aqui que, no futuro, despejaremos a saida do dominio.
    ui.set_console_text(
        "Ramsey-Putnam — console grafico\n\
         --------------------------------\n\
         (aqui aparece tudo que iria para o CLI)\n\
         \n\
         R(3,3) = 6\n"
            .into(),
    );

    ui.run()
}
