use iced::{
    widget::{button, column, container, row, text, Column, Row},
    Alignment, Element, Length,
};
use crate::ui::Message;

pub struct HomePage;

impl HomePage {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<'static, Message> {
        let title = text("🍉 Komorice")
            .size(32);

        let subtitle = text("A Komorebi GUI Ricing Configurator")
            .size(16);

        let komorebi_button = button(text("Edit Komorebi Configuration"))
            .padding(10)
            .on_press(Message::EditKomorebiConfig);

        let whkd_button = button(text("Edit Whkd Configuration"))
            .padding(10)
            .on_press(Message::EditWhkdConfig);

        let button_row = row![
            komorebi_button,
            whkd_button,
        ]
        .spacing(20)
        .padding(20);

        let content = column![
            title,
            subtitle,
            button_row,
        ]
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}