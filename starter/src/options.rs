use clap::Parser;

#[derive(Parser)]
#[command(name = "starter")]
#[command(author = "Kr328")]
#[command(version = "1.0.0")]
#[command(about = "Starter of Clash for Desktop.")]
pub struct Options {
    #[arg(long, default_value = "")]
    pub base_directory: String,

    #[arg(long, default_value_t = false)]
    pub no_shortcut: bool,

    #[arg(long, default_value_t = false)]
    pub hide_window: bool,
}
