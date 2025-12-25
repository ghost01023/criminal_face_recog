use crate::components::{GlassButton, GlassImageViewer, GlassInputLabel};
use crate::database::CriminalDB;
use crate::entities::criminal;
use crate::{Message, Page};

use iced::widget::{column, container, row, space, text};
use iced::{Alignment, Color, Element, Length, Task};
use std::sync::Arc;

pub struct ImageFindPage {
    pub selected_image: Vec<String>,
    pub is_identifying: bool,
    pub show_details: bool,
    pub identified_data: Option<criminal::Model>,
    pub not_found: bool, // New field to track search failure
}

impl Default for ImageFindPage {
    fn default() -> Self {
        Self {
            selected_image: Vec::new(),
            is_identifying: false,
            show_details: false,
            identified_data: None,
            not_found: false,
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
                self.not_found = false;
                self.is_identifying = true; // Start "Identifying..." status

                return Task::done(Message::IdentifyCriminalImage(first_path));
            }

            Message::Identity(criminal_id_str) => {
                let id = criminal_id_str.parse::<u32>().unwrap_or(0);

                if let Some(database) = db {
                    return Task::perform(
                        async move { database.get_criminal(id).await },
                        |result| match result {
                            Ok(Some(model)) => Message::IdentityDataLoaded(model),
                            _ => Message::IdentityError("Not Found".to_string()),
                        },
                    );
                }
            }

            Message::IdentityDataLoaded(model) => {
                self.is_identifying = false;
                self.show_details = true;
                self.not_found = false;
                self.identified_data = Some(model);
            }

            Message::IdentityError(_) => {
                self.is_identifying = false;
                self.show_details = false;
                self.not_found = true; // Trigger the "NOT FOUND" view
            }

            _ => {}
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'static, Message> {
        let left_content: Element<Message> = if self.selected_image.is_empty() {
            column![
                GlassInputLabel::new("NO TARGET LOADED").size(20),
                space().height(15.0),
                GlassButton::new("Upload Criminal Image").on_press(Message::OpenFilePicker),
            ]
            .align_x(Alignment::Center)
            .into()
        } else {
            let viewer = GlassImageViewer::new(self.selected_image.clone(), 0);
            viewer.view(Message::NextImage, Message::PrevImage).into()
        };

        let left_side = container(left_content)
            .width(Length::FillPortion(60))
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        // Branching for the Right Side Layout
        let right_content: Element<Message> = if self.is_identifying {
            self.loading_view()
        } else if self.not_found {
            self.not_found_view()
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
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        row![left_side, right_side].into()
    }

    fn not_found_view(&self) -> Element<'static, Message> {
        column![
            text("NOT FOUND IN DATABASE")
                .size(32)
                .color(Color::from_rgb(0.9, 0.4, 0.4)), // Red alert color
            space().height(20.0),
            text("The scanned face does not match any registered records.")
                .size(14)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.6)),
            space().height(40.0),
            GlassButton::new("Try Another Image").on_press(Message::OpenFilePicker),
            space().height(10.0),
            GlassButton::new("Go to Registry").on_press(Message::GoTo(Page::Registry)),
        ]
        .align_x(Alignment::Center)
        .into()
    }

    fn details_view(&self, data: &criminal::Model) -> Element<'static, Message> {
        column![
            text("CRIMINAL IDENTIFIED")
                .size(24)
                .color(Color::from_rgb(0.4, 0.9, 0.5)),
            space().height(20.0),
            self.info_field("CRIMINAL ID", data.criminal_id.to_string()),
            space().height(20.0),
            self.info_field("NAME", data.name.clone()),
            space().height(20.0),
            self.info_field(
                "FATHER'S NAME",
                data.fathers_name
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
            ),
            space().height(20.0),
            self.info_field("VIOLATIONS", data.no_of_crimes.to_string()),
            space().height(20.0),
            GlassButton::new("New Search").on_press(Message::OpenFilePicker),
            space().height(10.0),
            GlassButton::new("← Main Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_x(Alignment::Start)
        .into()
    }

    fn info_field(&self, label: &'static str, value: String) -> Element<'static, Message> {
        column![
            text(label)
                .size(11)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.4)),
            space().height(4.0),
            text(value).size(18).color(Color::WHITE),
        ]
        .into()
    }

    fn loading_view(&self) -> Element<'static, Message> {
        column![
            text("IDENTIFYING...")
                .size(28)
                .color(Color::from_rgb(0.4, 0.9, 0.5)),
            space().height(20.0),
            container(space().height(2.0))
                .width(250.0)
                .style(|_theme| LoaderBarStyle::style()),
        ]
        .align_x(Alignment::Center)
        .into()
    }

    fn awaiting_input_view(&self) -> Element<'static, Message> {
        column![
            text("Awaiting Image Input").color(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
            space().height(20.0),
            GlassButton::new("← Back to Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_x(Alignment::Center)
        .into()
    }
}

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
