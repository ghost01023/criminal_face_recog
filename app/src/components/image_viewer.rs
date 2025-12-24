use iced::{
    widget::{column, container, image, text, Container, Renderer, Space, Theme},
    Background, Border, Color, Element, Length, Shadow,
};

use crate::components::button::GlassButton;
pub struct GlassImageViewer {
    paths: Vec<String>,
    current_index: usize,
}

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
    pub fn new(paths: Vec<String>, current_index: usize) -> Self {
        Self {
            paths,
            current_index,
        }
    }

    /// We now take the messages as arguments so the component remains reusable
    pub fn view<Message>(
        self,
        on_next: Message,
        on_prev: Message,
    ) -> Container<'static, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let next_button = GlassButton::new("Next").on_press(on_next);
        let prev_button = GlassButton::new("Prev").on_press(on_prev);

        let content: Element<'static, Message, Theme, Renderer> = if self.paths.is_empty() {
            text("No Images Loaded")
                .size(20)
                .style(Color::from_rgba(0.6, 0.6, 0.6, 0.8))
                .into()
        } else {
            let path = self.paths.get(self.current_index).unwrap_or(&self.paths[0]);

            column![
                Space::with_height(10),
                image(path)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(iced::ContentFit::Contain),
                Space::with_height(10),
                // Putting buttons in a row or column under the image
                iced::widget::row![prev_button, next_button].spacing(10)
            ]
            .align_items(iced::Alignment::Center)
            .spacing(10)
            .into()
        };

        container(content)
            .width(Length::Fixed(600.0))
            .height(Length::Fixed(450.0)) // Increased height for buttons
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(
                CustomContainerStyle,
            )))
    }
}
