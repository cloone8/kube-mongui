use eframe::egui;

use crate::KubeMonGUI;

fn namespace_selector(config: &mut KubeMonGUI, ui: &mut egui::Ui) {
    let mut selected_namespace = config.selected_namespace.lock();

    let mut local_selected_namespace = selected_namespace.clone();

    egui::ComboBox::from_label("Selected namespace")
        .selected_text(match local_selected_namespace {
            Some(ref ns) => ns,
            None => "Select a namespace...",
        })
        .show_ui(ui, |ui| {
            let data = config.namespaces.lock();

            for item in data.iter() {
                ui.selectable_value(&mut local_selected_namespace, Some(item.to_owned()), item);
            }
        }
    );

    *selected_namespace = local_selected_namespace;
}

pub(crate) fn show(config: &mut KubeMonGUI, _: &egui::Context, ui: &mut egui::Ui) {
    namespace_selector(config, ui);
}
