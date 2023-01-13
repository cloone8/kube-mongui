use eframe::egui;

use crate::{KubeMonGUI, data::cronjob::CronJobInfo};

fn namespace_selector(config: &mut KubeMonGUI, ui: &mut egui::Ui) {
    let mut selected_namespace = config.selected_namespace.lock();

    let mut local_selected_namespace = selected_namespace.clone();

    ui.horizontal(|ui| {
        egui::ComboBox::from_label("Selected namespace")
            .selected_text(match local_selected_namespace {
                Some(ref ns) => ns,
                None => "All namespaces",
            })
            .show_ui(ui, |ui| {
                let data = config.namespaces.lock();

                for item in data.iter() {
                    ui.selectable_value(&mut local_selected_namespace, Some(item.to_owned()), item);
                }
            });

        // Short circuiting makes sure the button is only shown when a namespace is selected
        if local_selected_namespace.is_some() && ui.button("All namespaces").clicked() {
            local_selected_namespace = None;
        }
    });

    *selected_namespace = local_selected_namespace;
}

fn render_cronjob(cronjob: &CronJobInfo, ui: &mut egui::Ui) {
    let cj_header = format!("{} ({})", cronjob.name, cronjob.namespace.as_ref().unwrap_or(&String::from("None")));

    ui.collapsing(cj_header.as_str(), |ui| {
        cronjob.schedule.as_ref().map(|s| ui.label(format!("Schedule: {}", s)));
        cronjob.concurrent.as_ref().map(|c| ui.label(format!("Concurrency policy: {}", c)));

        if let Some(s) = &cronjob.status {
            s.last_schedule_time.map(|lst| ui.label(format!("Last schedule time: {}", lst)));
            s.last_successful_time.map(|lst| ui.label(format!("Last successful time: {}", lst)));
        }

        if let Some(jt) = &cronjob.job_template {
            jt.parallelism.map(|p| ui.label(format!("Parallelism: {}", p)));
            jt.containers.iter().for_each(|container_info| {
                ui.collapsing(container_info.name.as_str(), |ui| {
                    container_info.image.as_ref().map(|img| ui.label(format!("Image: {}", img)));

                    if !container_info.args.is_empty() {
                        ui.collapsing("Args:", |ui| {
                            container_info.args.iter().for_each(|a| {
                                ui.label(a);
                            });
                        });
                    }
                });
            });
        }
    });
}

pub(crate) fn show(config: &mut KubeMonGUI, _: &egui::Context, ui: &mut egui::Ui) {
    namespace_selector(config, ui);

    ui.separator();

    let data = config.cronjobs.lock();

    data.iter().for_each(|cj| render_cronjob(cj, ui));
}
