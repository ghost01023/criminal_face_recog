use iced::widget::{button, text, Button};
use iced::{Background, Border, Color, Element, Shadow, Theme};

pub struct GlassButton<'a, Message> {
    label: &'a str,
    on_press: Option<Message>,
}

impl<'a, Message: Clone + 'a> GlassButton<'a, Message> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            on_press: None,
        }
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn view(self) -> Button<'a, Message> {
        let btn = button(
            text(self.label)
                .size(16)
                // Use .color() directly for text in 0.14 to avoid inference errors
                .color(Color::from_rgb(0.9, 0.9, 0.9)),
        )
        .padding([12, 24])
        // 0.14 styling: Use a closure that receives (Theme, Status)
        .style(|_theme: &Theme, status: button::Status| {
            let base_style = button::Style {
                snap: true,
                background: Some(Background::Color(Color::from_rgba(0.2, 0.9, 0.5, 0.15))),
                border: Border {
                    color: Color::from_rgba(0.4, 0.9, 0.5, 0.3),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Color::from_rgb(0.4, 0.9, 0.5),
                shadow: Shadow::default(),
            };

            match status {
                button::Status::Hovered => button::Style {
                    background: Some(Background::Color(Color::from_rgba(0.2, 0.9, 0.5, 0.25))),
                    ..base_style
                },
                button::Status::Pressed => button::Style {
                    background: Some(Background::Color(Color::from_rgba(0.2, 0.9, 0.5, 0.35))),
                    ..base_style
                },
                button::Status::Disabled => button::Style {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    text_color: Color::from_rgba(0.4, 0.9, 0.5, 0.2),
                    ..base_style
                },
                button::Status::Active => base_style,
            }
        });

        if let Some(msg) = self.on_press {
            btn.on_press(msg)
        } else {
            btn
        }
    }
}

impl<'a, Message: Clone + 'a> From<GlassButton<'a, Message>> for Element<'a, Message> {
    fn from(button: GlassButton<'a, Message>) -> Self {
        button.view().into()
    }
}
