use iced::{
    widget::{button, text, Button},
    Background, Border, Color, Element, Shadow, Vector,
};

pub struct GlassButton<'a, Message> {
    label: &'a str,
    on_press: Option<Message>,
}

// 1. Define a struct for the style
struct CustomButtonStyle;

// 2. Implement the StyleSheet trait for the struct
impl button::StyleSheet for CustomButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.2, 0.9, 0.5, 0.15))),
            border: Border {
                color: Color::from_rgba(0.4, 0.9, 0.5, 0.3),
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Color::from_rgb(0.4, 0.9, 0.5),
            shadow_offset: Vector::ZERO,
            shadow: Shadow::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.2, 0.9, 0.5, 0.25))),
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.2, 0.9, 0.5, 0.35))),
            ..active
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color::from_rgba(0.4, 0.9, 0.5, 0.2),
            ..active
        }
    }
}

impl<'a, Message: Clone> GlassButton<'a, Message> {
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
                .style(Color::from_rgb(0.9, 0.9, 0.9)),
        )
        .padding([12, 24])
        // 3. Use the custom style
        .style(iced::theme::Button::custom(CustomButtonStyle));

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
