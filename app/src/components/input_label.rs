use iced::{widget::text, Color, Element};

pub struct GlassInputLabel<'a> {
    label: &'a str,
    size: u16,
    color: Color,
}

impl<'a> GlassInputLabel<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            size: 14,                              // Default size
            color: Color::from_rgb(0.4, 0.9, 0.5), // Default neon green
        }
    }

    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }

    pub fn view<Message: 'static>(self) -> Element<'a, Message> {
        text(self.label).size(self.size).style(self.color).into()
    }
}

impl<'a, Message: 'static> From<GlassInputLabel<'a>> for Element<'a, Message> {
    fn from(input_label: GlassInputLabel<'a>) -> Self {
        input_label.view()
    }
}
