use iced::{
    widget::{text_input, TextInput},
    Background, Border, Color, Element,
};

pub struct GlassTextInput<'a, Message> {
    placeholder: &'a str,
    value: &'a str,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
}

struct CustomInputStyle;

impl text_input::StyleSheet for CustomInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgba(0.1, 0.1, 0.1, 0.3)),
            border: Border {
                color: Color::from_rgba(0.4, 0.9, 0.5, 0.25),
                width: 1.0,
                radius: 8.0.into(),
            },
            icon_color: Color::from_rgb(0.6, 0.6, 0.6),
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let mut active = self.active(style);
        active.border.color = Color::from_rgba(0.4, 0.9, 0.5, 0.6);
        active
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.7, 0.7, 0.7, 0.5)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.4, 0.9, 0.5, 0.3)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.5, 0.5, 0.5, 0.5)
    }
}

// ... (Struct and CustomInputStyle implementation stay the same)

impl<'a, Message> GlassTextInput<'a, Message>
where
    Message: Clone + 'a, // Added 'a bound here
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
        F: 'static + Fn(String) -> Message,
    {
        self.on_input = Some(Box::new(f));
        self
    }

    pub fn view(self) -> TextInput<'a, Message> {
        let input = text_input(self.placeholder, self.value)
            .padding(12)
            .size(16)
            // The Box here defaults to + 'static.
            // Since CustomInputStyle is a unit struct, this is fine.
            .style(iced::theme::TextInput::Custom(Box::new(CustomInputStyle)));

        if let Some(on_input) = self.on_input {
            input.on_input(on_input)
        } else {
            input
        }
    }
}
impl<'a, Message: Clone + 'a> From<GlassTextInput<'a, Message>> for Element<'a, Message> {
    fn from(input: GlassTextInput<'a, Message>) -> Self {
        input.view().into()
    }
}
