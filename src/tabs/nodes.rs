use std::fmt::Display;

use eframe::egui::{self, Ui};

use crate::{KubeMonGUI, data::node::Hardware};

fn show_hardware_info(hardware: &Hardware, ui: &mut Ui) {
    hardware.cpu.map(|x| ui.label(format!("CPU: {}", x)));
    hardware.memory.map(|x| ui.label(format!("CPU: {}", x)));
    hardware.pods.map(|x| ui.label(format!("CPU: {}", x)));
    hardware.ephemeral_storage.map(|x| ui.label(format!("CPU: {}", x)));
    hardware.hugepages_2_mi.map(|x| ui.label(format!("CPU: {}", x)));
}

#[inline]
fn fmt_opt(opt: Option<impl Display>) -> String {
    match opt {
        Some(x) => x.to_string(),
        None => "?".to_string()
    }
}

fn show_combined_hardware_info(capacity: &Hardware, allocatable: &Hardware, ui: &mut Ui) {
    ui.label(format!("CPU: {}/{}", fmt_opt(allocatable.cpu), fmt_opt(capacity.cpu)));
    ui.label(format!("Memory: {}/{}", fmt_opt(allocatable.memory), fmt_opt(capacity.memory)));
    ui.label(format!("Pods: {}/{}", fmt_opt(allocatable.pods), fmt_opt(capacity.pods)));
    ui.label(format!("Ephemeral Storage: {}/{}", fmt_opt(allocatable.ephemeral_storage), fmt_opt(capacity.ephemeral_storage)));
    ui.label(format!("Hugepages 2Mi: {}/{}", fmt_opt(allocatable.hugepages_2_mi), fmt_opt(capacity.hugepages_2_mi)));
}

pub(crate) fn show(config: &mut KubeMonGUI, _: &egui::Context, ui: &mut egui::Ui) {
    let nodes = config.nodes.lock();

    for node in nodes.iter() {
        ui.collapsing(node.name.as_str(), |ui| {

            ui.label(node.hardware.arch.as_str());
            ui.collapsing("OS Info", |ui| {
                ui.label(node.os.name.as_str());

                node.os.image.as_ref().map(|image| ui.label(image));
                node.os.kernel_version.as_ref().map(|kver| ui.label(kver));
            });

            if node.hardware.capacity.is_some() && node.hardware.allocatable.is_some() {
                ui.collapsing("Hardware (Allocatable/Total)", |ui|
                    show_combined_hardware_info(
                        node.hardware.capacity.as_ref().unwrap(),
                        node.hardware.allocatable.as_ref().unwrap(), ui
                    )
                );
            } else if let Some(capacity) = &node.hardware.capacity {
                ui.collapsing("Hardware (Total)", |ui| show_hardware_info(capacity, ui));
            } else if let Some(allocatable) = &node.hardware.allocatable {
                ui.collapsing("Hardware (Allocatable)", |ui| show_hardware_info(allocatable, ui));
            }
        });
    }
}
