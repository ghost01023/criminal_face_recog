use crate::components::GlassButton;
use crate::components::GlassInputLabel;
use crate::components::VideoViewer;
use crate::database::CriminalDB;
use crate::entities::criminal;
use crate::{Message, Page};

use iced::widget::container;
use iced::{
    widget::{column, row, text, Space},
    Alignment, Color, Element, Length, Task,
};
use iced_video_player::Video;
use std::path::PathBuf;
use std::sync::Arc;
use url::Url;

pub struct VideoFindPage {
    pub selected_video: Option<String>,
    pub video: Option<Video>, // Store the Video
    pub is_scanning: bool,
    pub show_details: bool,
    pub identified_data: Option<criminal::Model>,
}

impl Default for VideoFindPage {
    fn default() -> Self {
        Self {
            selected_video: None,
            video: None,
            is_scanning: false,
            show_details: false,
            identified_data: None,
        }
    }
}

impl VideoFindPage {
    pub fn update(&mut self, message: Message, db: Option<Arc<CriminalDB>>) -> Task<Message> {
        match message {
            Message::FilesSelected(paths) => {
                if let Some(path) = paths.first() {
                    let path_str = path.to_string_lossy().to_string();

                    // Load the video
                    let path_buf = PathBuf::from(&path_str);
                    if let Ok(url) = Url::from_file_path(&path_buf) {
                        match Video::new(&url) {
                            Ok(video) => {
                                self.selected_video = Some(path_str.clone());
                                self.video = Some(video);
                                self.is_scanning = true;
                                self.show_details = false;
                                self.identified_data = None;

                                return Task::done(Message::IdentifyCriminalVideo(path_str));
                            }
                            Err(err) => {
                                eprintln!("Failed to load video: {:?}", err);
                            }
                        }
                    } else {
                        eprintln!("Failed to convert path to URL");
                    }
                }
            }

            Message::Identity(criminal_id_str) => {
                self.is_scanning = true;
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
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        // --- LEFT SIDE: 60% Width ---
        let left_content: Element<Message> =
            if let (Some(ref video), Some(ref path)) = (&self.video, &self.selected_video) {
                // Create VideoViewer on the fly with references
                VideoViewer::new(video, path.clone()).view()
            } else {
                column![
                    GlassInputLabel::new("NO VIDEO SOURCE").size(20),
                    GlassButton::new("Select Video File").on_press(Message::OpenFilePicker),
                ]
                .align_x(Alignment::Center)
                .spacing(15)
                .into()
            };

        let left_side = container(left_content)
            .width(Length::FillPortion(60))
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        // --- RIGHT SIDE: 40% Width ---
        let right_content: Element<Message> = if self.is_scanning {
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
            .center_x(Length::Fill);

        row![left_side, right_side].into()
    }

    fn details_view(&self, data: &criminal::Model) -> Element<'_, Message> {
        column![
            text("TARGET IDENTIFIED")
                .size(24)
                .color(Color::from_rgb(0.4, 0.8, 1.0)),
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
            GlassButton::new("Scan New Video").on_press(Message::OpenFilePicker),
            GlassButton::new("← Main Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_x(Alignment::Start)
        .spacing(10)
        .into()
    }

    fn info_field(&self, label: &'static str, value: String) -> Element<'_, Message> {
        column![
            text(label)
                .size(11)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.4)),
            text(value).size(18).color(Color::WHITE),
            Space::new().height(8.0),
        ]
        .into()
    }

    fn scanning_view(&self) -> Element<'_, Message> {
        column![
            text("SCANNING VIDEO...")
                .size(22)
                .color(Color::from_rgba(0.4, 0.8, 1.0, 1.0)),
            Space::new().height(10.0),
            text("Cross-referencing database frames")
                .size(14)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
            Space::new().height(20.0),
            container(
                Space::new()
                    .width(Length::Fixed(240.0))
                    .height(Length::Fixed(4.0))
            )
            .width(Length::Fixed(240.0))
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(
                    0.4, 0.8, 1.0, 0.8,
                ))),
                ..Default::default()
            }),
            Space::new().height(20.0),
            GlassButton::new("Cancel Scan").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_x(Alignment::Center)
        .spacing(10)
        .into()
    }

    fn awaiting_input_view(&self) -> Element<'_, Message> {
        column![
            text("Awaiting Video Feed").color(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
            Space::new().height(15.0),
            GlassButton::new("← Back to Menu").on_press(Message::GoTo(Page::MainMenu)),
        ]
        .align_x(Alignment::Center)
        .spacing(10)
        .into()
    }
}
