use crate::Message;
use iced::widget::{button, column, container, image, row, space, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Theme};

pub struct GlassImageViewer {
    images: Vec<String>,
    current_index: usize,
}

impl GlassImageViewer {
    pub fn new(images: Vec<String>, current_index: usize) -> Self {
        println!("Images inside the glassimage viewer are:");
        for el in images.clone() {
            println!("{}", el);
        }
        Self {
            images,
            current_index,
        }
    }

    pub fn view(self, next_msg: Message, prev_msg: Message) -> Element<'static, Message> {
        let image_content: Element<Message> =
            if let Some(path) = self.images.get(self.current_index) {
                image(path.clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(iced::ContentFit::Contain)
                    .into()
            } else {
                container(text("No Image Data"))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
            };

        let controls = row![
            button(text("Prev"))
                .on_press(prev_msg)
                .style(Self::nav_button_style),
            space().width(20.0),
            button(text("Next"))
                .on_press(next_msg)
                .style(Self::nav_button_style),
        ]
        .width(Length::Fill)
        .padding(10)
        .align_y(Alignment::Center);

        let viewer_layout = column![
            container(image_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            space().height(20.0),
            container(controls)
                .width(Length::Fill)
                .height(Length::Shrink)
        ];

        container(viewer_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.05, 0.05, 0.05, 0.2))),
                border: Border {
                    color: Color::from_rgba(0.4, 0.9, 0.5, 0.2),
                    width: 1.0,
                    radius: 12.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn nav_button_style(
        _theme: &Theme,
        status: iced::widget::button::Status,
    ) -> iced::widget::button::Style {
        let base = iced::widget::button::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
            text_color: Color::WHITE,
            border: Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            ..Default::default()
        };

        match status {
            iced::widget::button::Status::Hovered { .. } => iced::widget::button::Style {
                background: Some(Background::Color(Color::from_rgba(0.4, 0.9, 0.5, 0.4))),
                ..base
            },
            _ => base,
        }
    }
}
