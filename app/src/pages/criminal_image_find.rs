use crate::components::{GlassButton, GlassImageViewer, GlassInputLabel};
use crate::database::CriminalDB;
use crate::entities::criminal;
use crate::{Message, Page};

// IMPORT CHANGE: container::Style replaces container::Appearance
use iced::widget::container;
use iced::{
    widget::{column, row, text, Space},
    Alignment, Color, Element, Length, Task,
};
use std::sync::Arc;

pub struct ImageFindPage {
    pub selected_image: Vec<String>,
    pub is_identifying: bool,
    pub show_details: bool,
    pub identified_data: Option<criminal::Model>,
}

impl Default for ImageFindPage {
    fn default() -> Self {
        Self {
            selected_image: Vec::new(),
            is_identifying: false,
            show_details: false,
            identified_data: None,
        }
    }
}

impl ImageFindPage {
    pub fn update(&mut self, message: Message, db: Option<Arc<CriminalDB>>) -> Task<Message> {
        match message {
            Message::FilesSelected(paths) => {
                let first_path = paths
                    .get(0)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();

                self.selected_image = paths
                    .into_iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();

                self.show_details = false;
                self.identified_data = None;

                return Task::done(Message::IdentifyCriminalImage(first_path));
            }

            Message::Identity(criminal_id_str) => {
                self.is_identifying = true;
                let id = criminal_id_str.parse::<u32>().unwrap_or(0);

                if let Some(database) = db {
                    return Task::perform(
                        async move { database.get_criminal(id).await },
                        |result| match result {
                            Ok(Some(model)) => Message::IdentityDataLoaded(model),
                            _ => Message::IdentityError("Criminal Record Not Found".to_string()),
                        },
                    );
                }
            }

            Message::IdentityDataLoaded(model) => {
                self.is_identifying = false;
                self.show_details = true;
                self.identified_data = Some(model);
            }

            Message::IdentityError(err) => {
                self.is_identifying = false;
                eprintln!("Database Error: {}", err);
            }

            _ => {}
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let left_content: Element<Message> = if self.selected_image.is_empty() {
            column![
                GlassInputLabel::new("NO TARGET LOADED").size(20),
                GlassButton::new("Upload Criminal Image").on_press(Message::OpenFilePicker),
            ]
            .align_x(Alignment::Center)
            .spacing(15)
            .into()
        } else {
            // Create the viewer
            let viewer = GlassImageViewer::new(self.selected_image.clone(), 0);
            // Call view and into() - iced 0.14 handles the 'static transition here
            viewer.view(Message::NextImage, Message::PrevImage).into()
        }; // 0.14: Centering now requires Length::Fill
        let left_side = container(left_content)
            .width(Length::FillPortion(60))
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        let right_content: Element<Message> = if self.is_identifying {
            self.loading_view()
        } else if self.show_details {
            if let Some(data) = &self.identified_data {
                self.details_view(data)
            } else {
                text("Data synchronization error").into()
            }
        } else {
            self.awaiting_input_view()
        };

        let right_side = container(right_content)
            .width(Length::FillPortion(40))
            .height(Length::Fill)
            .padding(40)
            .center_x(Length::Fill);

        row![left_side, right_side].into()
    }

    fn details_view(&self, data: &criminal::Model) -> Element<'_, Message> {
        column![
            text("CRIMINAL IDENTIFIED")
                .size(24)
                .color(Color::from_rgb(0.4, 0.9, 0.5)),
            self.info_field("CRIMINAL ID", data.criminal_id.to_string()),
            self.info_field("NAME", data.name.clone()),
            self.info_field(
                "FATHER'S NAME",
                data.fathers_name
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
            ),
            self.info_field("VIOLATIONS", data.no_of_crimes.to_string()),
            self.info_field("DATE OF ARREST", data.date_of_arrest.to_string()),
            self.info_field(
                "LAST KNOWN LOCATION",
                data.arrested_location
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string())
            ),
            GlassButton::new("New Search").on_press(Message::OpenFilePicker),
            GlassButton::new("← Main Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_x(Alignment::Start)
        .into()
    }

    fn info_field(&self, label: &'static str, value: String) -> Element<'_, Message> {
        column![
            text(label)
                .size(11)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.4)),
            text(value).size(18).color(Color::WHITE),
        ]
        .into()
    }

    fn loading_view(&self) -> Element<'_, Message> {
        column![
            text("Running Recognition...")
                .size(22)
                .color(Color::from_rgb(0.4, 0.9, 0.5)),
            container(Space::new().height(2.0))
                .width(200.0)
                .style(|_theme| LoaderBarStyle::style()),
        ]
        .align_x(Alignment::Center)
        .into()
    }

    fn awaiting_input_view(&self) -> Element<'_, Message> {
        column![
            text("Awaiting Image Input").color(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
            GlassButton::new("← Back to Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_x(Alignment::Center)
        .into()
    }
}

// 0.14 Style Update: Struct now returns container::Style
struct LoaderBarStyle;
impl LoaderBarStyle {
    fn style() -> container::Style {
        container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.4, 0.9, 0.5, 0.8,
            ))),
            ..container::Style::default()
        }
    }
}
