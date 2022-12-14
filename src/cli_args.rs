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

    #[arg(value_enum, short, long, default_value_t = LogLevel::Warn)]
    pub verbosity: LogLevel
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

impl From<&Theme> for eframe::Theme {
    fn from(theme: &Theme) -> Self {
        match theme {
            Theme::Light => eframe::Theme::Light,
            Theme::Dark => eframe::Theme::Dark,
        }
    }
}
