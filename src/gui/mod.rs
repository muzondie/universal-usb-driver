use iced::{Application, Command, Element, Settings, Theme};
use iced::widget::{Column, Text, ProgressBar, Button};

pub struct MainState {
    current_status: String,
    progress: f32,
    installed_count: usize,
}

impl Application for MainState {
    type Executor = iced::executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {
            current_status: "Ready".into(),
            progress: 0.0,
            installed_count: 0,
        }, Command::none())
    }

    fn title(&self) -> String {
        "Universal USB Driver".into()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("Connected Devices:"))
            .push(ProgressBar::new(0.0..=100.0, self.progress))
            .push(Button::new("Scan").on_press(()))
            .push(Text::new(format!("Drivers installed: {}", self.installed_count)))
            .into()
    }
}