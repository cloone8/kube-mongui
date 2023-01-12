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

fn render_container(container: &crate::data::container::ContainerInfo, ui: &mut egui::Ui) {
    ui.collapsing(container.name.as_str(), |ui| {
        ui.label(container.name.as_str());
        ui.label(container.image.as_str());

        match container.status {
            crate::data::container::ContainerStatus::Running => ui.label("Running"),
            crate::data::container::ContainerStatus::Waiting => ui.label("Waiting"),
            crate::data::container::ContainerStatus::Terminated => ui.label("Terminated"),
        };

        let mut fake_bool = container.ready; // This is so the user won't be able to change the value
        ui.checkbox(&mut fake_bool, "Ready");

        if let Some(res) = container.resources.as_ref() {
            ui.collapsing("Resources", |ui| {
                if let Some(req) = res.requests.as_ref() {
                    ui.collapsing("Requests", |ui| {
                        if let Some(cpu) = req.cpu.as_ref() {
                            ui.label(format!("CPU: {}", cpu));
                        };

                        if let Some(mem) = req.memory.as_ref() {
                            ui.label(format!("Memory: {}", mem));
                        };
                    });
                }

                if let Some(lim) = res.limits.as_ref() {
                    ui.collapsing("Limits", |ui| {
                        if let Some(cpu) = lim.cpu.as_ref() {
                            ui.label(format!("CPU: {}", cpu));
                        };

                        if let Some(mem) = lim.memory.as_ref() {
                            ui.label(format!("Memory: {}", mem));
                        };
                    });
                }
            });
        };
    });
}

fn render_pod(pod: &crate::data::pod::PodInfo, ui: &mut egui::Ui) {
    ui.collapsing(pod.name.as_str(), |ui| {
        match pod.status {
            crate::data::pod::PodStatus::Running => ui.label("Running"),
            crate::data::pod::PodStatus::Pending => ui.label("Pending"),
            crate::data::pod::PodStatus::Succeeded => ui.label("Succeeded"),
            crate::data::pod::PodStatus::Failed => ui.label("Failed"),
            crate::data::pod::PodStatus::Unknown => ui.label("Unknown"),
        };

        match pod.qos_class {
            crate::data::pod::QoSClass::Guaranteed => ui.label("Guaranteed"),
            crate::data::pod::QoSClass::Burstable => ui.label("Burstable"),
            crate::data::pod::QoSClass::BestEffort => ui.label("BestEffort"),
        };

        ui.collapsing("Containers", |ui| {
            for container in pod.containers.iter() {
                render_container(container, ui);
            }
        });
    });
}

fn pods(config: &mut KubeMonGUI, ui: &mut egui::Ui) {
    let pods = config.pods.lock();

    for pod in pods.iter() {
        render_pod(pod, ui);
    }
}

pub(crate) fn show(config: &mut KubeMonGUI, _: &egui::Context, ui: &mut egui::Ui) {
    namespace_selector(config, ui);

    ui.separator();

    pods(config, ui);
}
