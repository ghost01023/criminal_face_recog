use crate::components::GlassButton;
use crate::components::GlassImageViewer;
use crate::components::GlassInputLabel;
use crate::components::GlassTextInput;
use crate::Page;

use crate::database::CriminalDB;
use crate::Message;
use iced::{
    widget::{column, container, row, scrollable, space},
    Alignment, Element, Length, Task,
};
use std::sync::Arc;

pub struct RegistryPage {
    pub name: String,
    pub fathers_name: String,
    pub no_of_crimes: String,
    pub arrested_location: String,
    pub name_error: bool,
    pub selected_images: Vec<String>,
    pub current_img_idx: usize,
    pub is_saving: bool,
    pub save_success: bool,
}

impl Default for RegistryPage {
    fn default() -> Self {
        Self {
            name: String::new(),
            fathers_name: String::new(),
            no_of_crimes: String::new(),
            arrested_location: String::new(),
            name_error: false,
            selected_images: Vec::new(),
            current_img_idx: 0,
            is_saving: false,
            save_success: false,
        }
    }
}

impl RegistryPage {
    // UPDATED: Now returns Task<Message>
    pub fn update(&mut self, message: Message, db: Option<Arc<CriminalDB>>) -> Task<Message> {
        match message {
            Message::NameChanged(value) => {
                self.name = value;
                self.name_error = false;
            }

            Message::FathersNameChanged(value) => {
                self.fathers_name = value;
            }

            Message::CrimesCountChanged(value) => {
                self.no_of_crimes = value;
            }

            Message::LocationChanged(value) => {
                self.arrested_location = value;
            }

            Message::SubmitForm => {
                if self.name.trim().is_empty() {
                    self.name_error = true;
                    return Task::none();
                }

                let Some(db) = db else {
                    return Task::none();
                };

                self.is_saving = true;

                let name = self.name.clone();
                let f_name = (!self.fathers_name.is_empty()).then(|| self.fathers_name.clone());
                let loc =
                    (!self.arrested_location.is_empty()).then(|| self.arrested_location.clone());
                let crimes = self.no_of_crimes.parse::<u32>().unwrap_or(1);
                let photo_paths = self.selected_images.clone();

                println!("ATTEMPTING TO ADD TO DATABASE");
                // UPDATED: Command::perform -> Task::perform
                return Task::perform(
                    async move {
                        // 1. Save to Rust Database
                        let criminal_id = db
                            .add_criminal(name, f_name, loc, crimes)
                            .await
                            .map_err(|e| e.to_string())?;

                        // 2. Save photos (preserving existing logic)
                        for path in &photo_paths {
                            if let Ok(bytes) = std::fs::read(path) {
                                let _ = db.add_criminal_photo(criminal_id, bytes).await;
                            }
                        }

                        Ok((criminal_id, photo_paths))
                    },
                    |result| match result {
                        Ok((id, paths)) => Message::DatabaseSaved(id, paths),
                        Err(e) => Message::SaveResult(Err(e)),
                    },
                );
            }

            Message::DatabaseSaved(id, paths) => {
                let paths_str = paths.join("&");
                let python_cmd = format!("add {} {}", id, paths_str);

                // Use Task::done to hand the command over to the main Python logic
                return Task::done(Message::PythonInput(python_cmd));
            }

            Message::SaveResult(Ok(_)) => {
                self.is_saving = false;
                self.save_success = true;
            }

            Message::ResetForm => {
                *self = RegistryPage::default();
            }

            Message::FilesSelected(paths) => {
                self.selected_images = paths
                    .into_iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();
                self.current_img_idx = 0;
            }
            Message::NextImage => {
                if !self.selected_images.is_empty() {
                    self.current_img_idx = (self.current_img_idx + 1) % self.selected_images.len();
                }
            }
            Message::PrevImage => {
                if !self.selected_images.is_empty() {
                    if self.current_img_idx == 0 {
                        self.current_img_idx = self.selected_images.len() - 1;
                    } else {
                        self.current_img_idx -= 1;
                    }
                }
            }
            _ => {}
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let top_left_content: Element<Message> = if self.selected_images.is_empty() {
            column![
                GlassInputLabel::new("Suspect Images").size(20),
                space().height(10.0),
                GlassButton::new("Attach Photos").on_press(Message::OpenFilePicker),
            ]
            .align_x(Alignment::Center)
            .into()
        } else {
            self.image_viewer_logic()
        };

        let left_col = column![container(top_left_content)
            .width(Length::Fill)
            .height(Length::FillPortion(60))
            .center_x(Length::Fill)
            .center_y(Length::Fill),]
        .width(Length::FillPortion(40));

        let right_col = column![
            // Header Row with Back Button
            GlassInputLabel::new("Criminal Details").size(32),
            space().height(20.0),
            container(scrollable(column![
                self.field_group(
                    if self.name_error { "Name *" } else { "Name" },
                    &self.name,
                    Message::NameChanged
                ),
                space().height(20.0),
                self.field_group(
                    "Father's Name",
                    &self.fathers_name,
                    Message::FathersNameChanged
                ),
                space().height(20.0),
                self.field_group(
                    "No. of Violations",
                    &self.no_of_crimes,
                    Message::CrimesCountChanged
                ),
                space().height(20.0),
                self.field_group(
                    "Last Arrested Location",
                    &self.arrested_location,
                    Message::LocationChanged
                ),
            ])),
            row![GlassButton::new("← Back").on_press(Message::GoTo(Page::MainMenu)),]
                .padding(10)
                .align_y(Alignment::Center),
            // Footer Action Area
            container(if self.is_saving {
                GlassButton::new("Saving...").on_press(Message::None)
            } else if self.save_success {
                GlassButton::new("Saved! (Reset Form)").on_press(Message::ResetForm)
            } else {
                GlassButton::new("Save to Database").on_press(Message::SubmitForm)
            })
            .width(Length::Fill)
            .height(Length::FillPortion(20))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
        ]
        .width(Length::FillPortion(60));

        row![left_col, right_col].into()
    }

    fn image_viewer_logic(&self) -> Element<'static, Message> {
        GlassImageViewer::new(self.selected_images.clone(), self.current_img_idx)
            .view(Message::NextImage, Message::PrevImage)
    }

    fn field_group<'a>(
        &self,
        label: &'a str,
        value: &'a str,
        on_change: fn(String) -> Message,
    ) -> Element<'a, Message> {
        column![
            GlassInputLabel::new(label).size(12),
            space().height(8.0),
            GlassTextInput::new(label, value).on_input(on_change),
        ]
        .into()
    }
}
