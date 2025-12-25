use crate::Message;
use iced::widget::{column, container, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Theme};
use iced_video_player::{Video, VideoPlayer};

pub struct VideoViewer<'a> {
    video: &'a Video,
    path: String,
}

impl<'a> VideoViewer<'a> {
    pub fn new(video: &'a Video, path: String) -> Self {
        Self { video, path }
    }

    pub fn view(&self) -> Element<'a, Message> {
        // ✅ iced_video_player 0.6.0 PUBLIC API
        let player = VideoPlayer::new(self.video)
            .width(Length::Fill)
            .height(Length::Fill);

        container(
            column![
                player,
                text(format!("Source: {}", self.path))
                    .size(12)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
                    })
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.05, 0.05, 0.05, 1.0))),
            border: Border {
                color: Color::from_rgba(0.4, 0.8, 1.0, 0.2),
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}
