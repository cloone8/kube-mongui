mod kubeproxy;
mod updaters;
mod tabs;
mod util;
mod cli_args;
mod data;

use std::{sync::Arc, time::Duration, fmt::Display};

use clap::Parser;
use cli_args::CLIArgs;
use data::{pod::PodInfo, node::NodeInfo};
use eframe::{egui::{self, ScrollArea}, epaint::mutex::Mutex};
use kubeproxy::KubeProxy;

fn main() {
    let args = CLIArgs::parse();

    simple_logger::init_with_level(log::Level::from(args.verbosity.clone())).unwrap();

    println!("Starting kube-mongui");
    println!("Log level: {}", args.verbosity.to_string());

    let k8s_api_url = match args.kubeproxy_url {
        Some(ref url) => {
            log::info!("Using kubectl proxy url: {}", url);
            KubeUrl::Url(url.clone())
        },
        None => match kubeproxy::start_kubectl_proxy(args.port) {
            Ok(child) => {
                log::info!("Starting own kubectl proxy (with specified port: {})", if args.port != 0 { args.port.to_string() } else { "no".to_string() });
                KubeUrl::Proxy(child)
            },
            Err(e) => panic!("Failed to start kubectl proxy: {:?}", e),
        }
    };

    let mut ui = Box::new(KubeMonGUI::new(k8s_api_url));

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
    Resources,
    Nodes
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
            KubeMonTabs::Nodes => write!(f, "Nodes"),
        }
    }
}

pub(crate) enum KubeUrl {
    Proxy(KubeProxy),
    Url(String)
}

impl KubeUrl {
    pub fn get_url(&self) -> &str {
        match self {
            KubeUrl::Proxy(proxy) => proxy.url.as_str(),
            KubeUrl::Url(url) => url.as_str()
        }
    }
}

pub(crate) struct KubeMonGUI {
    k8s_api: KubeUrl,

    selected_tab: KubeMonTabs,

    namespaces: Arc<Mutex<Vec<String>>>,
    selected_namespace: Arc<Mutex<Option<String>>>,

    pods: Arc<Mutex<Vec<PodInfo>>>,

    nodes: Arc<Mutex<Vec<NodeInfo>>>
}

impl KubeMonGUI {
    fn new(k8s_api: KubeUrl) -> Self {
        KubeMonGUI {
            k8s_api,
            selected_tab: KubeMonTabs::default(),
            namespaces: Arc::new(Mutex::new(Vec::new())),
            selected_namespace: Arc::new(Mutex::new(None)),
            pods: Arc::new(Mutex::new(Vec::new())),
            nodes: Arc::new(Mutex::new(Vec::new()))
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
            ui.selectable_value(&mut config.selected_tab, KubeMonTabs::Nodes, KubeMonTabs::Nodes.to_string());
        }
    );
}

impl eframe::App for KubeMonGUI {
   fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("kube-mongui");

            tab_selector(self, ui);

            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                match self.selected_tab {
                    KubeMonTabs::RunningPods => tabs::pods::show(self, ctx, ui),
                    KubeMonTabs::CronJobs => (),
                    KubeMonTabs::Resources => (),
                    KubeMonTabs::Nodes => tabs::nodes::show(self, ctx, ui),
                }
            });

            // Update at least once per second
            ctx.request_repaint_after(Duration::from_secs(1));
        });
   }
}
