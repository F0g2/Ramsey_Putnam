// Build script: roda ANTES de compilar o nosso codigo.
// Aqui ele transforma a UI declarativa (ui/app.slint) em codigo Rust,
// que depois e trazido de volta com slint::include_modules!().
fn main() {
    slint_build::compile("ui/app.slint").expect("falha ao compilar a UI Slint");
}
