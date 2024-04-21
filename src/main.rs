use iced::Application;
pub mod app;
pub mod components;
pub mod view;
pub mod types;
pub mod messages;
pub mod theme;
pub mod widget;

fn main() -> Result<(), iced::Error> {
    // run the app from main function
    app::ReceiptCalculator::run(iced::Settings::default())
}

