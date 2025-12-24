use iced::widget::{column, container, text};
use iced::{Alignment, Color, Element, Length, Renderer, Theme};

pub struct VideoViewer {
    path: String,
    is_looping: bool,
}

impl VideoViewer {
    pub fn new(path: String) -> Self {
        Self {
            path,
            is_looping: true,
        }
    }

    pub fn view<Message>(&self) -> Element<'static, Message, Theme, Renderer>
    where
        Message: 'static,
    {
        // In a production iced app, you would use a dedicated crate
        // like `iced_video_player`. For this UI structure, we represent the video
        // container where the frames would be rendered.
        container(
            column![
                text("VIDEO PLAYBACK")
                    .size(12)
                    .style(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
                text(format!("Source: {}", self.path)).size(14),
                text(if self.is_looping { "Mode: Looping" } else { "" }).size(12),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(VideoBoxStyle)))
        .into()
    }
}

struct VideoBoxStyle;
impl iced::widget::container::StyleSheet for VideoBoxStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.05, 0.05, 0.05, 1.0,
            ))),
            ..Default::default()
        }
    }
}
