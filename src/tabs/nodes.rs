use std::fmt::Display;

use chrono::{Local, DateTime};
use eframe::egui::{self, Ui};

use crate::{KubeMonGUI, data::node::Hardware};

fn show_hardware_info(hardware: &Hardware, ui: &mut Ui) {
    hardware.cpu.map(|x| ui.label(format!("CPU: {}", x)));
    hardware.memory.map(|x| ui.label(format!("CPU: {}", x)));
    hardware.pods.map(|x| ui.label(format!("CPU: {}", x)));
    hardware.ephemeral_storage.map(|x| ui.label(format!("CPU: {}", x)));
    hardware.hugepages_2_mi.map(|x| ui.label(format!("CPU: {}", x)));
}

fn fmt_opt(opt: Option<impl Display>) -> String {
    match opt {
        Some(x) => x.to_string(),
        None => "?".to_string()
    }
}

fn fmt_cpu(cpu: f64) -> String {
    format!("{:.2} cores", cpu)
}

fn fmt_cpu_opt(cpu: Option<f64>) -> String {
    match cpu {
        Some(x) => fmt_cpu(x),
        None => "?".to_string()
    }
}

fn fmt_mem_opt(mem: Option<i64>) -> String {
    match mem {
        Some(x) => fmt_mem(x),
        None => "?".to_string()
    }
}

fn fmt_mem(mem: i64) -> String {
    format!("{} MiB", mem / 1024 / 1024)
}

fn show_combined_hardware_info(capacity: &Hardware, allocatable: &Hardware, ui: &mut Ui) {
    ui.label(format!("CPU: {}/{}", fmt_cpu_opt(allocatable.cpu), fmt_cpu_opt(capacity.cpu)));
    ui.label(format!("Memory: {}/{}", fmt_mem_opt(allocatable.memory), fmt_mem_opt(capacity.memory)));
    ui.label(format!("Pods: {}/{}", fmt_opt(allocatable.pods), fmt_opt(capacity.pods)));
    ui.label(format!("Ephemeral Storage: {}/{}", fmt_mem_opt(allocatable.ephemeral_storage), fmt_mem_opt(capacity.ephemeral_storage)));
    ui.label(format!("Hugepages 2Mi: {}/{}", fmt_opt(allocatable.hugepages_2_mi), fmt_opt(capacity.hugepages_2_mi)));
}

pub(crate) fn show(config: &mut KubeMonGUI, _: &egui::Context, ui: &mut egui::Ui) {
    let nodes = config.nodes.lock();

    for node in nodes.iter() {
        ui.collapsing(node.name.as_str(), |ui| {

            ui.collapsing("OS Info", |ui| {
                ui.label(node.os.name.as_str());
                ui.label(node.hardware.arch.as_str());
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

            if node.usage.is_some() && node.hardware.allocatable.is_some() {
                ui.collapsing("Node Usage", |ui| {
                    let usage = node.usage.as_ref().unwrap();
                    let allocatable = node.hardware.allocatable.as_ref().unwrap();

                    let last_updated = DateTime::<Local>::from(usage.updated).to_string();

                    let cpu_used = usage.cpu.zip(allocatable.cpu).map(|(usage, allocatable)| usage / allocatable);
                    let mem_used = usage.memory.zip(allocatable.memory).map(|(usage, allocatable)| usage as f64 / allocatable as f64);

                    ui.label(format!("Last Updated: {}", last_updated));

                    if let Some(cpu_used) = cpu_used {
                        let progress_cpu = egui::ProgressBar::new(cpu_used as f32)
                            .text(format!("CPU: {}/{}", fmt_cpu(usage.cpu.unwrap()), fmt_cpu(allocatable.cpu.unwrap())));

                        ui.add(progress_cpu);
                    }

                    if let Some(mem_used) = mem_used {
                        let progress_mem = egui::ProgressBar::new(mem_used as f32)
                            .text(format!("Memory: {}/{}", fmt_mem(usage.memory.unwrap()), fmt_mem(allocatable.memory.unwrap())));

                        ui.add(progress_mem);
                    }
                });
            }

            if !node.addresses.is_empty() {
                ui.collapsing("Addresses", |ui| {
                    for address in node.addresses.iter() {
                        ui.label(format!("{}: {}", address.address_type, address.address));
                    }
                });
            }

            if !node.conditions.is_empty() {
                ui.collapsing("Conditions", |ui| {
                    for condition in node.conditions.iter() {
                        let condition_label = format!("{}: {}", condition.condition_type, if condition.status.is_some() { condition.status.unwrap() } else { false });

                        ui.collapsing(condition_label, |ui| {
                            condition
                            .last_transition_time
                            .as_ref()
                            .map(|last_transition_time|
                                ui.label(format!("Last transition time: {}", DateTime::<Local>::from(*last_transition_time)))
                            );

                        condition
                            .last_heartbeat_time
                            .as_ref()
                            .map(|last_heartbeat_time|
                                ui.label(format!("Last heartbeat time: {}", DateTime::<Local>::from(*last_heartbeat_time)))
                            );

                        condition
                            .reason
                            .as_ref()
                            .map(|reason|
                                ui.label(format!("Reason: {}", reason))
                            );

                        condition
                            .message
                            .as_ref()
                            .map(|message|
                                ui.label(format!("Message: {}", message))
                            );
                        });
                    }
                });
            }
        });
    }
}
