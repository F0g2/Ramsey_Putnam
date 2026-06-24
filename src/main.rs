// Ponto de entrada. Fica curto de proposito: so a blindagem geral de erro.
// Toda a logica vive em app::run().

mod app;

fn main() {
    // Blindagem: se run() falhar por qualquer motivo, mostramos a mensagem
    // e encerramos com codigo de erro, em vez de estourar um panic feio.
    if let Err(erro) = app::run() {
        eprintln!("erro fatal: {erro}");
        std::process::exit(1);
    }
}
