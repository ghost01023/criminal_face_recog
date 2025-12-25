use crate::components::GlassButton;
use crate::components::GlassInputLabel;
use crate::components::VideoViewer;
use crate::database::CriminalDB;
use crate::entities::criminal;
use crate::{Message, Page};

use iced::{
    widget::{column, container, row, text, Space},
    Alignment, Color, Command, Element, Length, Renderer, Theme,
};
use std::sync::Arc;

pub struct VideoFindPage {
    pub selected_video: Option<String>,
    pub is_scanning: bool,
    pub show_details: bool,
    pub identified_data: Option<criminal::Model>,
}

impl Default for VideoFindPage {
    fn default() -> Self {
        Self {
            selected_video: None,
            is_scanning: false,
            show_details: false,
            identified_data: None,
        }
    }
}

impl VideoFindPage {
    pub fn update(&mut self, message: Message, db: Option<Arc<CriminalDB>>) -> Command<Message> {
        match message {
            Message::FilesSelected(paths) => {
                if let Some(path) = paths.first() {
                    let path_str = path.to_string_lossy().to_string();
                    self.selected_video = Some(path_str.clone());
                    self.is_scanning = true;
                    self.show_details = false;
                    self.identified_data = None;

                    // Trigger the Python command via the main update loop
                    return Command::perform(async {}, move |_| {
                        Message::IdentifyCriminalVideo(path_str)
                    });
                }
            }

            Message::Identity(criminal_id_str) => {
                self.is_scanning = true; // Still scanning/loading until DB results arrive
                let id = criminal_id_str.parse::<u32>().unwrap_or(0);

                if let Some(database) = db {
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
                self.is_scanning = false;
                self.show_details = true;
                self.identified_data = Some(model);
            }

            Message::IdentityError(err) => {
                self.is_scanning = false;
                eprintln!("Database Error: {}", err);
            }

            _ => {}
        }
        Command::none()
    }

    pub fn view(&self) -> Element<'static, Message> {
        // --- LEFT SIDE: 60% Width (Video Display) ---
        let left_content: Element<Message, Theme, Renderer> =
            if let Some(path) = &self.selected_video {
                VideoViewer::new(path.clone()).view()
            } else {
                column![
                    GlassInputLabel::new("NO VIDEO SOURCE").size(20),
                    GlassButton::new("Select Video File").on_press(Message::OpenFilePicker),
                ]
                .align_items(Alignment::Center)
                .spacing(15)
                .into()
            };

        let left_side = container(left_content)
            .width(Length::FillPortion(60))
            .height(Length::Fill)
            .center_x()
            .center_y();

        // --- RIGHT SIDE: 40% Width ---
        let right_content: Element<'static, Message, Theme, Renderer> = if self.is_scanning {
            self.scanning_view()
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
            text("TARGET IDENTIFIED")
                .size(24)
                .style(Color::from_rgb(0.4, 0.8, 1.0)), // Blue-ish for video scan success
            Space::with_height(30),
            self.info_field("CRIMINAL ID", data.criminal_id.to_string()),
            self.info_field("NAME", data.name.clone()),
            self.info_field(
                "FATHER'S NAME",
                data.fathers_name
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
            ),
            self.info_field(
                "CRIMINAL HISTORY",
                format!("{} known violations", data.no_of_crimes)
            ),
            self.info_field("LAST CAPTURE", data.date_of_arrest.to_string()),
            self.info_field(
                "LAST KNOWN LOCATION",
                data.arrested_location
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string())
            ),
            Space::with_height(40),
            GlassButton::new("Scan New Video").on_press(Message::OpenFilePicker),
            Space::with_height(10),
            GlassButton::new("← Main Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_items(Alignment::Start)
        .into()
    }

    fn info_field(&self, label: &'static str, value: String) -> Element<'static, Message> {
        column![
            text(label)
                .size(11)
                .style(Color::from_rgba(1.0, 1.0, 1.0, 0.4)),
            text(value).size(18).style(Color::WHITE),
            Space::with_height(12),
        ]
        .into()
    }

    fn scanning_view(&self) -> Element<'static, Message> {
        column![
            text("SCANNING VIDEO...")
                .size(22)
                .style(Color::from_rgba(0.4, 0.8, 1.0, 1.0)),
            Space::with_height(10),
            text("Cross-referencing database frames")
                .size(14)
                .style(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
            Space::with_height(20),
            container(Space::with_height(Length::Fixed(2.0)))
                .width(Length::Fixed(240.0))
                .style(iced::theme::Container::Custom(Box::new(ScanningBarStyle))),
            Space::with_height(40),
            GlassButton::new("Cancel Scan").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_items(Alignment::Center)
        .into()
    }

    fn awaiting_input_view(&self) -> Element<'static, Message> {
        column![
            text("Awaiting Video Feed").style(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
            Space::with_height(20),
            GlassButton::new("← Back to Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}

struct ScanningBarStyle;
impl iced::widget::container::StyleSheet for ScanningBarStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.4, 0.8, 1.0, 0.8,
            ))),
            ..Default::default()
        }
    }
}
