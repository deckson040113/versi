use iced::window;

mod app;
mod message;
mod settings;
mod state;
mod theme;
mod views;
mod widgets;

fn main() -> iced::Result {
    iced::application(app::FnmUi::title, app::FnmUi::update, app::FnmUi::view)
        .subscription(app::FnmUi::subscription)
        .theme(app::FnmUi::theme)
        .window(window::Settings {
            size: iced::Size::new(800.0, 600.0),
            min_size: Some(iced::Size::new(600.0, 400.0)),
            ..Default::default()
        })
        .run_with(app::FnmUi::new)
}
