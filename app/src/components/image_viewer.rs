use iced::{
    widget::{container, text, Container},
    Background, Border, Color, Element, Length, Shadow,
};

pub struct GlassImageViewer;

struct CustomContainerStyle;

impl container::StyleSheet for CustomContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.15, 0.15, 0.15, 0.4))),
            border: Border {
                color: Color::from_rgba(0.4, 0.9, 0.5, 0.2),
                width: 1.0,
                radius: 12.0.into(),
            },
            text_color: None,
            shadow: Shadow::default(),
        }
    }
}

impl GlassImageViewer {
    pub fn new() -> Self {
        Self
    }

    pub fn view<Message: 'static>(self) -> Container<'static, Message> {
        container(
            text("🖼️ Image Placeholder")
                .size(20)
                .style(Color::from_rgba(0.6, 0.6, 0.6, 0.8)),
        )
        .width(Length::Fixed(300.0))
        .height(Length::Fixed(200.0))
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(
            CustomContainerStyle,
        )))
    }
}

impl<Message: 'static> From<GlassImageViewer> for Element<'static, Message> {
    fn from(viewer: GlassImageViewer) -> Self {
        viewer.view().into()
    }
}
