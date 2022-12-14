mod kubeproxy;
mod updaters;
mod tabs;
mod util;
mod cli_args;
mod data;

use std::{sync::Arc, time::Duration, fmt::Display};

use clap::Parser;
use cli_args::CLIArgs;
use data::pod::PodInfo;
use eframe::{egui, epaint::mutex::Mutex};
use kubeproxy::KubeProxy;

fn main() {
    let args = CLIArgs::parse();

    let kubeproxy = match kubeproxy::start_kubectl_proxy(args.port) {
        Ok(child) => child,
        Err(e) => panic!("Failed to start kubectl proxy: {:?}", e),
    };

    let mut ui = Box::new(KubeMonGUI::new(kubeproxy));

    match updaters::start_all(&mut ui) {
        Ok(_) => (),
        Err(e) => panic!("Failed to start updater: {:?}", e),
    };

    eframe::run_native(
        "kube-mongui",
        get_native_options(&args),
        Box::new(|_| ui)
    );
}

fn get_native_options(args: &CLIArgs) -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();

    match &args.theme {
        Some(prefered_theme) => {
            native_options.follow_system_theme = false;

            native_options.default_theme = eframe::Theme::from(prefered_theme);
        },
        None => native_options.follow_system_theme = true,
    };

    native_options
}

#[derive(PartialEq, Eq, Clone)]
enum KubeMonTabs {
    RunningPods,
    CronJobs,
    Resources
}

impl Default for KubeMonTabs {
    fn default() -> Self {
        KubeMonTabs::RunningPods
    }
}

impl Display for KubeMonTabs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KubeMonTabs::RunningPods => write!(f, "Pods"),
            KubeMonTabs::CronJobs => write!(f, "CronJobs"),
            KubeMonTabs::Resources => write!(f, "Resource usage"),
        }
    }
}

pub(crate) struct KubeMonGUI {
    proxy: KubeProxy,

    selected_tab: KubeMonTabs,

    namespaces: Arc<Mutex<Vec<String>>>,
    selected_namespace: Arc<Mutex<Option<String>>>,

    pods: Arc<Mutex<Vec<PodInfo>>>,
}

impl KubeMonGUI {
    fn new(proxy: KubeProxy) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        KubeMonGUI {
            proxy,
            selected_tab: KubeMonTabs::default(),
            namespaces: Arc::new(Mutex::new(Vec::new())),
            selected_namespace: Arc::new(Mutex::new(None)),
            pods: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

fn tab_selector(config: &mut KubeMonGUI, ui: &mut egui::Ui) {
    egui::ComboBox::from_label("Tab")
        .selected_text(config.selected_tab.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut config.selected_tab, KubeMonTabs::RunningPods, KubeMonTabs::RunningPods.to_string());
            ui.selectable_value(&mut config.selected_tab, KubeMonTabs::CronJobs, KubeMonTabs::CronJobs.to_string());
            ui.selectable_value(&mut config.selected_tab, KubeMonTabs::Resources, KubeMonTabs::Resources.to_string());
        }
    );
}

impl eframe::App for KubeMonGUI {
   fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("kube-mongui");

            tab_selector(self, ui);

            ui.separator();

            match self.selected_tab {
                KubeMonTabs::RunningPods => tabs::pods::show(self, ctx, ui),
                KubeMonTabs::CronJobs => (),
                KubeMonTabs::Resources => (),
            }

            // Update at least once per second
            ctx.request_repaint_after(Duration::from_secs(1));
        });
   }
}
