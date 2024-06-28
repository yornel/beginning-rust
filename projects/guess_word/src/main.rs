use iced::{Application, Settings};
mod gui;   // src/gui.rs の場合は mod gui;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400, 600);
    gui::GuessWord::run(settings)            // src/gui.rs の場合は gui::GuessWord::run(settings);
}