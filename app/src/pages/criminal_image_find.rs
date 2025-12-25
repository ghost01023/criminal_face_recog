use crate::components::GlassButton;
use crate::components::GlassImageViewer;
use crate::components::GlassInputLabel;
use crate::database::CriminalDB;
use crate::entities::criminal;
use crate::{Message, Page};

use iced::{
    widget::{column, container, row, text, Space},
    Alignment, Color, Command, Element, Length, Renderer, Theme,
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
    pub fn update(&mut self, message: Message, db: Option<Arc<CriminalDB>>) -> Command<Message> {
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

                return Command::perform(async {}, move |_| {
                    Message::IdentifyCriminalImage(first_path)
                });
            }

            Message::Identity(criminal_id_str) => {
                self.is_identifying = true;
                let id = criminal_id_str.parse::<u32>().unwrap_or(0);
                println!("Attempting to find details for id {}", id);
                if let Some(database) = db {
                    println!("Connecting to db");
                    return Command::perform(
                        async move { database.get_criminal(id).await },
                        |result| match result {
                            Ok(Some(model)) => Message::IdentityDataLoaded(model),
                            _ => Message::IdentityError("Criminal Record Not Found".to_string()),
                        },
                    );
                }
            }

            Message::IdentityDataLoaded(model) => {
                println!("LOADED....");
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
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        // --- LEFT SIDE: 60% Width (Image Display) ---
        let left_content: Element<Message, Theme, Renderer> = if self.selected_image.is_empty() {
            column![
                GlassInputLabel::new("NO TARGET LOADED").size(20),
                GlassButton::new("Upload Criminal Image").on_press(Message::OpenFilePicker),
            ]
            .align_items(Alignment::Center)
            .spacing(15)
            .into()
        } else {
            GlassImageViewer::new(self.selected_image.clone(), 0)
                .view(Message::NextImage, Message::PrevImage)
                .into()
        };

        let left_side = container(left_content)
            .width(Length::FillPortion(60))
            .height(Length::Fill)
            .center_x()
            .center_y();

        // --- RIGHT SIDE: 40% Width (Details / Status) ---
        let right_content: Element<Message, Theme, Renderer> = if self.is_identifying {
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
            .center_x();

        row![left_side, right_side].into()
    }

    fn details_view(&self, data: &criminal::Model) -> Element<'static, Message> {
        column![
            text("CRIMINAL IDENTIFIED")
                .size(24)
                .style(Color::from_rgb(0.4, 0.9, 0.5)),
            Space::with_height(30),
            // We pass OWNED Strings now, not references to temporary strings
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
            Space::with_height(40),
            GlassButton::new("New Search").on_press(Message::OpenFilePicker),
            Space::with_height(10),
            GlassButton::new("← Main Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_items(Alignment::Start)
        .into()
    }

    // 2. Change 'value' to an owned String and return 'static
    fn info_field(&self, label: &'static str, value: String) -> Element<'static, Message> {
        column![
            text(label)
                .size(11)
                .style(Color::from_rgba(1.0, 1.0, 1.0, 0.4)),
            text(value).size(18).style(Color::WHITE), // text() accepts String and takes ownership
            Space::with_height(12),
        ]
        .into()
    }
    fn loading_view(&self) -> Element<Message> {
        column![
            text("Running Recognition...")
                .size(22)
                .style(Color::from_rgb(0.4, 0.9, 0.5)),
            Space::with_height(20),
            container(Space::with_height(2.0))
                .width(200.0)
                .style(iced::theme::Container::Custom(Box::new(LoaderBarStyle))),
        ]
        .align_items(Alignment::Center)
        .into()
    }

    fn awaiting_input_view(&self) -> Element<Message> {
        column![
            text("Awaiting Image Input").style(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
            Space::with_height(20),
            GlassButton::new("← Back to Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}

struct LoaderBarStyle;
impl iced::widget::container::StyleSheet for LoaderBarStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.4, 0.9, 0.5, 0.8,
            ))),
            ..Default::default()
        }
    }
}
