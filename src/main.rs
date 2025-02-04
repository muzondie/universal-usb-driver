mod gui;
mod driver_db;
mod detection;
mod installation;

use iced::Settings;
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> iced::Result {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();
    gui::MainState::run(Settings::default())
}