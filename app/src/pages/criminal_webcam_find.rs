use crate::components::{GlassButton, GlassImageViewer, GlassInputLabel};
use crate::database::CriminalDB;
use crate::entities::criminal;
use crate::webcam_task::capture_frame;
use crate::Message;
use crate::Page;

use iced::widget::{column, container, row, space, text};
use iced::{Alignment, Background, Color, Element, Length, Task};
use iced_video_player::{Video, VideoPlayer};
use std::sync::Arc;
// Add this import for URL handling
use url::Url;

pub struct WebcamFindPage {
    pub is_webcam_on: bool,
    pub last_captured_frame: Option<String>,
    pub identified_data: Option<criminal::Model>,
    pub is_identifying: bool,
    pub not_found: bool,
    pub video_source: Option<Video>,
    pub unknown_count: u32,
    pub suspect_photos: Vec<String>,
    pub current_photo_index: usize,
    pub db: Option<Arc<CriminalDB>>,
}

impl Default for WebcamFindPage {
    fn default() -> Self {
        Self {
            is_webcam_on: false,
            last_captured_frame: None,
            identified_data: None,
            is_identifying: false,
            not_found: false,
            video_source: None,
            unknown_count: 0,
            suspect_photos: Vec::new(),
            current_photo_index: 0,
            db: None,
        }
    }
}
// ... (Default remains the same) ...

impl WebcamFindPage {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, message: Message, db: Option<Arc<CriminalDB>>) -> Task<Message> {
        match message {
            Message::ToggleWebcam(on) => {
                self.is_webcam_on = on;
                if on {
                    self.unknown_count = 0;
                    self.not_found = false;
                    self.identified_data = None;
                    let camera_uri = Url::parse("v4l2:///dev/video0").unwrap();
                    self.video_source = Video::new(&camera_uri).ok();
                } else {
                    self.video_source = None;
                    self.is_identifying = false;
                }
            }

            // Triggered every 2 seconds by main.rs subscription
            Message::TickWebcam => {
                if self.is_webcam_on
                    && !self.is_identifying
                    && self.identified_data.is_none()
                    && self.unknown_count < 10
                {
                    // 1. Release /dev/video0 so Nokhwa can grab it
                    self.video_source = None;
                    // 2. Start capture task
                    return Task::perform(capture_frame(), Message::WebcamFrameCaptured);
                }
            }

            Message::WebcamFrameCaptured(path) => {
                self.last_captured_frame = Some(path.clone());
                if path.is_empty() {
                    // If hardware failed to open, try to restart the player
                    return Task::done(Message::ToggleWebcam(true));
                }
                self.is_identifying = true;
                return Task::done(Message::IdentifyCriminalImage(path));
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
                self.is_webcam_on = false;
                self.video_source = None; // Stop camera on success
                let id = model.criminal_id;
                println!("IDENTITY DATA LOADED FOR WEBCAM!");
                if let Some(database) = db {
                    return Task::perform(
                        async move { database.get_criminal_with_photos(id).await },
                        |result| match result {
                            Ok(Some((model, photos))) => {
                                println!("OK WITH LOADED PHOTOS");
                                Message::IdentityDataLoadedWithPhotos(model, photos)
                            }
                            _ => Message::IdentityError("Record not found".to_string()),
                        },
                    );
                }
            }

            Message::IdentityDataLoadedWithPhotos(model, photos) => {
                self.current_photo_index = 0;

                let temp_dir = std::path::Path::new("temp_webcam_identify");
                let _ = std::fs::create_dir_all(temp_dir);
                println!("Now loading photos");

                self.suspect_photos = std::iter::once(self.last_captured_frame.clone())
                    .flatten() // Turns Option<String> into an iterator of 0 or 1 items
                    .chain(
                        photos
                            .into_iter()
                            .enumerate()
                            .filter_map(|(i, photo_model)| {
                                let file_name =
                                    format!("suspect_{}_{}.jpg", photo_model.criminal_id, i);
                                let file_path = temp_dir.join(file_name);

                                if std::fs::write(&file_path, photo_model.photo).is_ok() {
                                    Some(file_path.to_string_lossy().to_string())
                                } else {
                                    None
                                }
                            }),
                    )
                    .collect();
                println!("Suspect photos saved");
                for ele in &self.suspect_photos {
                    println!("{}", ele);
                }
                self.identified_data = Some(model);
            }

            Message::IdentityError(_) => {
                self.unknown_count += 1;
                self.is_identifying = false;
                println!("RECORD NOT FOUND");

                if self.unknown_count >= 10 {
                    self.not_found = true;
                    self.is_webcam_on = false;
                    self.video_source = None;
                } else {
                    // Re-enable live feed until the next TickWebcam (2 seconds)
                    let camera_uri = Url::parse("v4l2:///dev/video0").unwrap();
                    self.video_source = Video::new(&camera_uri).ok();
                }
            }

            Message::NextImage => {
                if !self.suspect_photos.is_empty() {
                    self.current_photo_index =
                        (self.current_photo_index + 1) % self.suspect_photos.len();
                    println!("INDEX IS NOW: {}", self.current_photo_index);
                }
                {
                    println!("Photos vector is empty");
                }
            }

            Message::PrevImage => {
                if !self.suspect_photos.is_empty() {
                    self.current_photo_index = if self.current_photo_index == 0 {
                        self.suspect_photos.len() - 1
                    } else {
                        self.current_photo_index - 1
                    };
                }
            }

            Message::ResetWebcamSearch => {
                return Task::done(Message::ToggleWebcam(true));
            }

            _ => {}
        }
        Task::none()
    }
    pub fn view(&self) -> Element<'_, Message> {
        // --- LEFT SIDE: 40% Width ---
        let left_content: Element<Message> = if self.identified_data.is_some() {
            // Show the static frame that matched
            if let Some(_path) = &self.last_captured_frame {
                println!("self.suspect_photos are");
                for el in self.suspect_photos.clone() {
                    println!("{}", el);
                }
                println!(
                    "IMAGE INDEX INSIDE WEBCAM_PAGE IS {}",
                    self.current_photo_index
                );
                GlassImageViewer::new(self.suspect_photos.clone(), self.current_photo_index)
                    .view(Message::NextImage, Message::PrevImage)
            } else {
                text("Match frame lost").into()
            }
        } else if let Some(source) = &self.video_source {
            // Show the live feed
            let player = VideoPlayer::new(source)
                .width(Length::Fill)
                .height(Length::Shrink);
            container(
                column![
                    player,
                    text("LIVE SCANNER ACTIVE")
                        .size(12)
                        .color(Color::from_rgba(0.4, 0.9, 0.5, 0.5))
                ]
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.0, 0.0, 0.0))),
                ..Default::default()
            })
            .into()
        } else {
            container(GlassButton::new("Initiate Webcam").on_press(Message::ToggleWebcam(true)))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        };

        // --- RIGHT SIDE: 60% Width ---
        // 1. Define the footer separately with an explicit type
        let footer_action: Element<'static, Message> = if self.identified_data.is_some()
            || self.not_found
        {
            container(GlassButton::new("Restart Scanner").on_press(Message::ResetWebcamSearch))
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into()
        } else {
            container(text("Keep subject in view...").color(Color::from_rgba(1.0, 1.0, 1.0, 0.3)))
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into()
        };

        // 2. Now use it inside your column! macro
        let right_content = column![
            row![
                GlassButton::new("← Back").on_press(Message::GoTo(Page::MainMenu)),
                space().width(Length::Fill),
                GlassInputLabel::new("AI LIVE SCAN").size(32),
            ]
            .align_y(Alignment::Center),
            space().height(40.0),
            // Middle section logic
            if self.is_identifying {
                self.status_view("ANALYZING FRAME...", Color::from_rgb(0.4, 0.8, 1.0))
            } else if self.not_found {
                self.status_view("SEARCH EXHAUSTED: UNKNOWN", Color::from_rgb(0.9, 0.4, 0.4))
            } else if let Some(data) = &self.identified_data {
                self.details_view(data)
            } else if self.is_webcam_on {
                self.status_view(
                    &format!("SCANNING (ATTEMPT {}/10)", self.unknown_count + 1),
                    Color::from_rgba(1.0, 1.0, 1.0, 0.5),
                )
            } else {
                self.status_view("SYSTEM READY", Color::from_rgba(1.0, 1.0, 1.0, 0.2))
            },
            space().height(Length::Fill),
            // 3. Insert the annotated variable here
            footer_action
        ]
        .padding(40)
        .align_x(Alignment::Center);
        row![
            container(left_content)
                .width(Length::FillPortion(40))
                .height(Length::Fill),
            container(right_content)
                .width(Length::FillPortion(60))
                .height(Length::Fill)
        ]
        .into()
    }

    fn status_view(&self, label: &str, color: Color) -> Element<'static, Message> {
        column![
            text(label.to_string()).size(28).color(color),
            space().height(20.0),
            container(space().height(2.0))
                .width(250.0)
                .style(move |_| container::Style {
                    background: Some(Background::Color(color)),
                    ..Default::default()
                })
        ]
        .align_x(Alignment::Center)
        .into()
    }

    fn details_view(&self, data: &criminal::Model) -> Element<'static, Message> {
        column![
            text("CRIMINAL IDENTIFIED")
                .size(24)
                .color(Color::from_rgb(0.4, 0.9, 0.5)),
            space().height(30.0),
            text(data.name.clone()).size(32).color(Color::WHITE),
            text(format!("ID: {}", data.criminal_id))
                .size(16)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
            space().height(10.0),
            text(format!("Violations: {} counts", data.no_of_crimes)).size(18),
        ]
        .align_x(Alignment::Start)
        .into()
    }
}
