use iced::widget::{text_input, TextInput};
use iced::{Background, Border, Color, Element, Theme};

pub struct GlassTextInput<'a, Message> {
    placeholder: &'a str,
    value: &'a str,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

impl<'a, Message> GlassTextInput<'a, Message>
where
    Message: Clone + 'a,
{
    pub fn new(placeholder: &'a str, value: &'a str) -> Self {
        Self {
            placeholder,
            value,
            on_input: None,
        }
    }

    pub fn on_input<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_input = Some(Box::new(f));
        self
    }

    pub fn view(self) -> TextInput<'a, Message> {
        let mut input = text_input(self.placeholder, self.value)
            .padding(12)
            .size(16)
            // 0.14 styling: Use a closure that returns text_input::Style based on Status
            .style(|_theme: &Theme, status: text_input::Status| {
                let active = text_input::Style {
                    background: Background::Color(Color::from_rgba(0.1, 0.1, 0.1, 0.3)),
                    border: Border {
                        color: Color::from_rgba(0.4, 0.9, 0.5, 0.25),
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    icon: Color::from_rgb(0.6, 0.6, 0.6),
                    placeholder: Color::from_rgba(0.7, 0.7, 0.7, 0.5),
                    value: Color::WHITE,
                    selection: Color::from_rgba(0.4, 0.9, 0.5, 0.3),
                };

                match status {
                    // 0.14: These are now struct variants, so we use { .. } to ignore the fields
                    text_input::Status::Focused { .. } | text_input::Status::Hovered { .. } => {
                        text_input::Style {
                            border: Border {
                                color: Color::from_rgba(0.4, 0.9, 0.5, 0.6),
                                ..active.border
                            },
                            ..active
                        }
                    }
                    _ => active,
                }
            });

        if let Some(on_input) = self.on_input {
            input = input.on_input(on_input);
        }

        input
    }
}

impl<'a, Message: Clone + 'a> From<GlassTextInput<'a, Message>> for Element<'a, Message> {
    fn from(input: GlassTextInput<'a, Message>) -> Self {
        input.view().into()
    }
}
