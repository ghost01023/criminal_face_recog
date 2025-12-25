use iced::widget::text;
use iced::{Color, Element};

pub struct GlassInputLabel<'a> {
    label: &'a str,
    size: u32, // Changed from u16 to u32
    color: Color,
}

impl<'a> GlassInputLabel<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            size: 14,
            color: Color::from_rgb(0.4, 0.9, 0.5),
        }
    }

    pub fn size(mut self, size: u32) -> Self {
        // Now accepts u32
        self.size = size;
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }

    pub fn view<Message: 'static>(self) -> Element<'a, Message> {
        // 1. Use .size(self.size) which now works because self.size is u32
        // 2. Use .color(self.color) instead of .style() for direct coloring
        text(self.label).size(self.size).color(self.color).into()
    }
}

impl<'a, Message: 'static> From<GlassInputLabel<'a>> for Element<'a, Message> {
    fn from(input_label: GlassInputLabel<'a>) -> Self {
        input_label.view()
    }
}
