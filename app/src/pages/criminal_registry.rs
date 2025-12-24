use crate::components::{
    button::GlassButton, image_viewer::GlassImageViewer, input_label::GlassInputLabel,
    text_input::GlassTextInput,
};
use crate::Message;
use iced::{
    widget::{column, container, row, scrollable, Space},
    Alignment, Element, Length, Renderer, Theme,
}; // Import the shared Message enum from main.rs
#[derive(Default)]
pub struct RegistryPage {
    pub name: String,
    pub fathers_name: String,
    pub no_of_crimes: String,
    pub arrested_location: String,
    pub name_error: bool,
    pub selected_images: Vec<String>,
    pub current_img_idx: usize,
}

impl RegistryPage {
    pub fn view(&self) -> Element<Message> {
        // --- LEFT COLUMN (40% Width) ---
        let top_left_content: Element<Message, Theme, Renderer> = if self.selected_images.is_empty()
        {
            column![
                GlassInputLabel::new("Suspect Images").size(20),
                GlassButton::new("Attach Photos").on_press(Message::OpenFilePicker),
            ]
            .align_items(Alignment::Center)
            .spacing(10)
            .into() // Converts Column to Element
        } else {
            self.image_viewer_logic().into() // Converts Viewer Container to Element
        };

        let left_col = column![
            container(top_left_content) // Now it knows exactly what this is
                .width(Length::Fill)
                .height(Length::FillPortion(60))
                .center_x()
                .center_y(),
            Space::with_height(Length::FillPortion(40)),
        ]
        .width(Length::FillPortion(40));
        // --- RIGHT COLUMN (60% Width) ---
        let right_col = column![
            container(scrollable(
                column![
                    GlassInputLabel::new("Criminal Details").size(24),
                    Space::with_height(20),
                    self.field_group(
                        if self.name_error { "Name *" } else { "Name" },
                        &self.name,
                        Message::NameChanged
                    ),
                    self.field_group(
                        "Father's Name",
                        &self.fathers_name,
                        Message::FathersNameChanged
                    ),
                    self.field_group(
                        "No. of Violations",
                        &self.no_of_crimes,
                        Message::CrimesCountChanged
                    ),
                    self.field_group(
                        "Last Arrested Location",
                        &self.arrested_location,
                        Message::LocationChanged
                    ),
                ]
                .spacing(20)
            ))
            .height(Length::FillPortion(80))
            .padding(40),
            container(GlassButton::new("Save to Database").on_press(Message::SubmitForm))
                .width(Length::Fill)
                .height(Length::FillPortion(20))
                .center_x()
                .center_y(),
        ]
        .width(Length::FillPortion(60));

        row![left_col, right_col].into()
    }

    fn image_viewer_logic(&self) -> Element<Message> {
        GlassImageViewer::new(self.selected_images.clone(), self.current_img_idx)
            .view(Message::NextImage, Message::PrevImage)
            .into()
    }

    fn field_group<'a>(
        &self,
        label: &'a str,
        value: &'a str,
        on_change: fn(String) -> Message,
    ) -> Element<'a, Message> {
        column![
            GlassInputLabel::new(label).size(12),
            GlassTextInput::new(label, value).on_input(on_change),
        ]
        .spacing(8)
        .into()
    }
}
