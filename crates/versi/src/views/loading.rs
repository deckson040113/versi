use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length};

use crate::message::Message;

pub fn view() -> Element<'static, Message> {
    container(
        column![text("Loading...").size(24),]
            .spacing(16)
            .align_x(Alignment::Center),
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
