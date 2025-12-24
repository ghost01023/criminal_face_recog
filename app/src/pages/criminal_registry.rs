use crate::components::GlassButton;
use crate::components::GlassImageViewer;
use crate::components::GlassInputLabel;
use crate::components::GlassTextInput;

use crate::database::CriminalDB;
use crate::Message;
use iced::{
    widget::{column, container, row, scrollable, Space},
    Alignment, Command, Element, Length, Renderer, Theme,
};
use std::sync::Arc;

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
    /// Decoupled logic for handling Registry-specific messages
    pub fn update(&mut self, message: Message, db: Option<Arc<CriminalDB>>) -> Command<Message> {
        match message {
            Message::NameChanged(s) => {
                self.name = s;
                self.name_error = false;
            }
            Message::FathersNameChanged(s) => self.fathers_name = s,
            Message::CrimesCountChanged(s) => self.no_of_crimes = s,
            Message::LocationChanged(s) => self.arrested_location = s,

            Message::NextImage => {
                if !self.selected_images.is_empty() {
                    self.current_img_idx = (self.current_img_idx + 1) % self.selected_images.len();
                }
            }
            Message::PrevImage => {
                if !self.selected_images.is_empty() {
                    self.current_img_idx = if self.current_img_idx == 0 {
                        self.selected_images.len() - 1
                    } else {
                        self.current_img_idx - 1
                    };
                }
            }

            Message::SubmitForm => {
                if self.name.trim().is_empty() {
                    self.name_error = true;
                    return Command::none();
                }

                let Some(db) = db else {
                    eprintln!("Database not connected yet.");
                    return Command::none();
                };

                // Clone data for async block
                let name = self.name.clone();
                let f_name = (!self.fathers_name.is_empty()).then(|| self.fathers_name.clone());
                let loc =
                    (!self.arrested_location.is_empty()).then(|| self.arrested_location.clone());
                let crimes = self.no_of_crimes.parse::<u32>().unwrap_or(1);
                let photo_paths = self.selected_images.clone();

                return Command::perform(
                    async move {
                        let criminal_id = db
                            .add_criminal(name, f_name, loc, crimes)
                            .await
                            .map_err(|e| e.to_string())?;

                        for path in photo_paths {
                            if let Ok(bytes) = std::fs::read(path) {
                                db.add_criminal_photo(criminal_id, bytes)
                                    .await
                                    .map_err(|e| e.to_string())?;
                            }
                        }
                        Ok(criminal_id)
                    },
                    Message::SaveResult,
                );
            }
            Message::SaveResult(Ok(_)) => {
                *self = RegistryPage::default(); // Reset form on success
            }

            Message::FilesSelected(paths) => {
                self.selected_images = paths
                    .into_iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();
                self.current_img_idx = 0;
            }
            _ => {}
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        // ... (Keep your existing view code here)
        let top_left_content: Element<Message, Theme, Renderer> = if self.selected_images.is_empty()
        {
            column![
                GlassInputLabel::new("Suspect Images").size(20),
                GlassButton::new("Attach Photos").on_press(Message::OpenFilePicker),
            ]
            .align_items(Alignment::Center)
            .spacing(10)
            .into()
        } else {
            self.image_viewer_logic().into()
        };

        let left_col = column![
            container(top_left_content)
                .width(Length::Fill)
                .height(Length::FillPortion(60))
                .center_x()
                .center_y(),
            Space::with_height(Length::FillPortion(40)),
        ]
        .width(Length::FillPortion(40));

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
