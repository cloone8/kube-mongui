mod kubeproxy;
mod updater;

use std::sync::Arc;

use eframe::{egui, epaint::mutex::Mutex};
use kubeproxy::KubeProxy;

fn get_native_options() -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();

    native_options.follow_system_theme = true;

    native_options
}

fn main() {
    let kubeproxy = match kubeproxy::start_kubectl_proxy(8001) {
        Ok(child) => child,
        Err(e) => panic!("Failed to start kubectl proxy: {:?}", e),
    };

    let mut ui = Box::new(KubeMonGUI::new(kubeproxy));

    match updater::start_all(&mut ui) {
        Ok(_) => (),
        Err(e) => panic!("Failed to start updater: {:?}", e),
    };

    eframe::run_native(
        "kube-mongui",
        get_native_options(),
        Box::new(|_| ui)
    );
}

pub(crate) struct KubeMonGUI {
    proxy: KubeProxy,

    namespaces: Arc<Mutex<Vec<String>>>,
    selected_namespace: Arc<Mutex<Option<String>>>,
}

impl KubeMonGUI {
    fn new(proxy: KubeProxy) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        KubeMonGUI {
            proxy: proxy,
            namespaces: Arc::new(Mutex::new(Vec::new())),
            selected_namespace: Arc::new(Mutex::new(None)),
        }
    }
}

impl KubeMonGUI {
    fn namespace_selector(&mut self, ui: &mut egui::Ui) {
        let mut selected_namespace = self.selected_namespace.lock();

        let mut local_selected_namespace = selected_namespace.clone();

        egui::ComboBox::from_label("Selected namespace")
            .selected_text(match local_selected_namespace {
                Some(ref ns) => ns,
                None => "Select a namespace...",
            })
            .show_ui(ui, |ui| {
                let data = self.namespaces.lock();

                for item in data.iter() {
                    ui.selectable_value(&mut local_selected_namespace, Some(item.to_owned()), item);
                }
            }
        );

        *selected_namespace = local_selected_namespace;
    }
}


impl eframe::App for KubeMonGUI {
   fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("kube-mongui");

            self.namespace_selector(ui);

            ui.label(format!("Listening on {}", self.proxy.listen_addr));
        });
   }
}