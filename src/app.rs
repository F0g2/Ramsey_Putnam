// Controle da aplicacao (a "cola" entre a UI e o dominio).
// include_modules!() traz para ca o codigo gerado a partir de ui/app.slint.
slint::include_modules!();

use slint::{ComponentHandle, ModelRc, VecModel};
use std::rc::Rc;

pub fn run() -> Result<(), slint::PlatformError> {
    // No wasm: o winit NAO anexa o canvas ao DOM por padrao (with_append=false),
    // entao a pagina fica em branco. Selecionamos o backend winit com um hook
    // que liga o append. So no web; no desktop usa o backend padrao.
    #[cfg(target_arch = "wasm32")]
    {
        use i_slint_backend_winit::winit::platform::web::WindowAttributesExtWebSys;
        let backend = i_slint_backend_winit::Backend::builder()
            .with_window_attributes_hook(|attrs| attrs.with_append(true))
            .build()
            .expect("falha ao criar o backend winit");
        slint::platform::set_platform(Box::new(backend)).expect("falha ao definir a plataforma");
    }

    // Cria a janela PRIMEIRO: e isso que inicializa o backend/plataforma do
    // Slint. O registro de fonte via fontique (abaixo) exige a plataforma ja
    // inicializada -- fazer antes daqui causa panic "platform not initialized".
    let ui = MainWindow::new()?;

    // Fonte monospace embutida no binario (DejaVu Sans Mono, licenca Bitstream
    // Vera). Vem depois de criar a janela e antes do run() (antes de renderizar).
    // Em Slint 1.17 isso passa pelo modulo fontique (feature instavel).
    {
        use std::sync::Arc;
        let dados = include_bytes!("../ui/assets/fonts/DejaVuSansMono.ttf");
        let blob = slint::fontique_010::fontique::Blob::new(Arc::new(dados.to_vec()));
        let mut colecao = slint::fontique_010::shared_collection();
        colecao.register_fonts(blob, None);
    }

    // Modelo das linhas do console. VecModel e uma lista observavel: quando
    // mudamos (set_vec/push), a UI se atualiza sozinha. Comeca vazio -> a UI
    // mostra a dica "Configure os parametros...".
    let linhas: Rc<VecModel<LinhaConsole>> = Rc::new(VecModel::default());
    ui.set_linhas(ModelRc::from(linhas.clone()));

    // O botao Executar: por ora monta um TRACE de demonstracao (sem calculo
    // real), lendo os parametros do global Estado pela API do Rust.
    let ui_weak = ui.as_weak();
    let linhas_exec = linhas.clone();
    ui.on_rodar(move || {
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };
        let est = ui.global::<Estado>();
        let s = est.get_s();
        let t = est.get_t();
        let n = est.get_n();

        linhas_exec.set_vec(vec![
            LinhaConsole { texto: format!("$ ramsey --n {n}  R({s},{t})").into(), tom: 2 },
            LinhaConsole { texto: format!("[init]  n = {n} vertices · alvo R({s},{t})").into(), tom: 0 },
            LinhaConsole { texto: format!("[scan]  enumerando 2-coloracoes de K_{n} ...").into(), tom: 0 },
            LinhaConsole { texto: "[nota]  calculo real ainda nao implementado (demo)".into(), tom: 3 },
            LinhaConsole { texto: "[ok]    interface pronta para receber o dominio".into(), tom: 1 },
        ]);
    });

    ui.run()
}
