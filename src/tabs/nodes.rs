use eframe::egui;

use crate::KubeMonGUI;

pub(crate) fn show(config: &mut KubeMonGUI, _: &egui::Context, ui: &mut egui::Ui) {
    let nodes = config.nodes.lock();

    for node in nodes.iter() {
        ui.collapsing(node.name.as_str(), |ui| {

            ui.label(node.arch.as_str());
            ui.collapsing("OS Info", |ui| {
                ui.label(node.os.name.as_str());

                node.os.image.as_ref().map(|image| ui.label(image));
                node.os.kernel_version.as_ref().map(|kver| ui.label(kver));
            });
        });
    }
}
