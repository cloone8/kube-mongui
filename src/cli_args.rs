use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub(crate) struct CLIArgs {
    /// The port that the kubectl API proxy will be running on. 0 means "let kubectl pick"
    #[arg(short, long, default_value_t = 0)]
    pub port: u16,

    /// The theme to pick. Will use system theme if not specified
    #[arg(value_enum, short, long)]
    pub theme: Option<Theme>,

    /// The update factor. This scales the update interval of all updaters. 1.0 means "normal speed", 2.0 means "twice as slow", etc.
    /// Most updaters will update their data every second, but not all.
    #[arg(short = 'f', long, default_value_t = 1.0)]
    pub update_factor: f64,

    #[cfg(not(debug_assertions))]
    #[arg(value_enum, short, long, default_value_t = LogLevel::Warn)]
    pub verbosity: LogLevel,

    #[cfg(debug_assertions)]
    #[arg(value_enum, short, long, default_value_t = LogLevel::Info)]
    pub verbosity: LogLevel,

    /// Url to the kubernetes API. Must be pre-authenticated. If left empty, kube-mongui will
    /// attempt to start a kubectl proxy by itself
    #[arg(short = 'u', long)]
    pub kubeproxy_url: Option<String>
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum Theme {
    Light,
    Dark
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum LogLevel {
        Error,
        Warn,
        Info,
        Debug,
        Trace,
}

#[cfg(debug_assertions)]
impl From<LogLevel> for log::Level {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Error => log::Level::Error,
            LogLevel::Warn => log::Level::Warn,
            LogLevel::Info => log::Level::Info,
            LogLevel::Debug => log::Level::Debug,
            LogLevel::Trace => log::Level::Trace,
        }
    }
}

#[cfg(not(debug_assertions))]
#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum LogLevel {
        Error,
        Warn,
        Info,
}

#[cfg(not(debug_assertions))]
impl From<LogLevel> for log::Level {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Error => log::Level::Error,
            LogLevel::Warn => log::Level::Warn,
            LogLevel::Info => log::Level::Info,
        }
    }
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Error => "error".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Info => "info".to_string(),
            #[cfg(debug_assertions)]
            LogLevel::Debug => "debug".to_string(),
            #[cfg(debug_assertions)]
            LogLevel::Trace => "trace".to_string(),
        }
    }
}

impl From<&Theme> for eframe::Theme {
    fn from(theme: &Theme) -> Self {
        match theme {
            Theme::Light => eframe::Theme::Light,
            Theme::Dark => eframe::Theme::Dark,
        }
    }
}
