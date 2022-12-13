use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub(crate) struct CLIArgs {
    #[arg(short, long, default_value_t = 0)]
    pub port: u16,

    #[arg(value_enum, short, long)]
    pub theme: Option<Theme>
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum Theme {
    Light,
    Dark
}

impl From<&Theme> for eframe::Theme {
    fn from(theme: &Theme) -> Self {
        match theme {
            Theme::Light => eframe::Theme::Light,
            Theme::Dark => eframe::Theme::Dark,
        }
    }
}
