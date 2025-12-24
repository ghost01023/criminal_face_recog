use crate::components::button::GlassButton;
use iced::{
    widget::{column, container, image, row, text, Container, Space},
    Background, Border, Color, Element, Length, Shadow,
};

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
    /// Creates a new GlassImageViewer with the given image paths and current index
    pub fn new(paths: Vec<String>, current_index: usize) -> Self {
        let index = if paths.is_empty() {
            0
        } else {
            current_index.min(paths.len() - 1)
        };
        Self {
            paths,
            current_index: index,
        }
    }

    /// Render the image viewer with next/prev buttons
    pub fn view<Message>(self, on_next: Message, on_prev: Message) -> Container<'static, Message>
    where
        Message: Clone + 'static,
    {
        let next_button = GlassButton::new("Next").on_press(on_next);
        let prev_button = GlassButton::new("Prev").on_press(on_prev);

        let content: Element<'static, Message> = if self.paths.is_empty() {
            text("No Images Loaded")
                .size(20)
                .style(Color::from_rgba(0.6, 0.6, 0.6, 0.8))
                .into()
        } else {
            let path = self.paths[self.current_index].clone();
            let counter = format!("{} / {}", self.current_index + 1, self.paths.len());

            column![
                Space::with_height(10),
                image(path)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(iced::ContentFit::Contain),
                Space::with_height(10),
                text(counter)
                    .size(14)
                    .style(Color::from_rgba(0.7, 0.7, 0.7, 0.9)),
                Space::with_height(10),
                row![prev_button, Space::with_width(10), next_button].spacing(10)
            ]
            .align_items(iced::Alignment::Center)
            .spacing(10)
            .into()
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(
                CustomContainerStyle,
            )))
    }
}
