use crate::components::{GlassButton, GlassImageViewer, GlassInputLabel};
use crate::database::CriminalDB;
use crate::entities::criminal;
use crate::{Message, Page};

use iced::widget::{column, container, row, space, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task, Theme};
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
}

// ... (Default remains the same) ...

impl WebcamFindPage {
    pub fn new() -> Self {
        Self {
            is_webcam_on: false,
            last_captured_frame: None,
            identified_data: None,
            is_identifying: false,
            not_found: false,
            video_source: None,
        }
    }
    pub fn update(&mut self, message: Message, _db: Option<Arc<CriminalDB>>) -> Task<Message> {
        match message {
            Message::ToggleWebcam(on) => {
                self.is_webcam_on = on;
                if on {
                    let camera_uri = Url::parse("v4l2:///dev/video0").unwrap();
                    self.video_source = Video::new(&camera_uri).ok();
                } else {
                    self.video_source = None;
                    self.is_identifying = false;
                }
            }

            Message::IdentityDataLoaded(model) => {
                self.is_identifying = false;
                self.identified_data = Some(model);
                self.not_found = false;
                self.is_webcam_on = false;
                self.video_source = None; // Drops camera handle
            }

            Message::TickWebcam => {
                if self.is_webcam_on && !self.is_identifying && self.identified_data.is_none() {
                    return Task::done(Message::CaptureWebcamFrame);
                }
            }
            Message::WebcamFrameCaptured(path) => {
                self.last_captured_frame = Some(path.clone());
                self.is_identifying = true;
                return Task::done(Message::PythonInput(format!("identify {}", path)));
            }

            Message::IdentityError(_) => {
                self.is_identifying = false;
                self.not_found = true;
            }

            Message::ResetWebcamSearch => {
                self.identified_data = None;
                self.not_found = false;
                self.is_webcam_on = true;
                let camera_uri = Url::parse("v4l2:///dev/video0").unwrap();
                self.video_source = Video::new(&camera_uri).ok();
            }

            // ... (TickWebcam, WebcamFrameCaptured, Identity handlers remain the same) ...
            _ => {}
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let left_content: Element<Message> = if let Some(_) = &self.identified_data {
            if let Some(path) = &self.last_captured_frame {
                GlassImageViewer::new(vec![path.clone()], 0).view(Message::None, Message::None)
            } else {
                text("Match frame lost").into()
            }
        } else if let Some(source) = &self.video_source {
            let player = VideoPlayer::new(source)
                .width(Length::Fill)
                .height(Length::Shrink);

            container(
                column![
                    player,
                    text("Live Feed: Active")
                        .size(12)
                        .style(|_theme| text::Style {
                            color: Some(Color::from_rgba(0.4, 0.9, 0.5, 0.5)),
                        })
                ]
                .spacing(10)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.05, 0.05, 0.05, 1.0))),
                border: Border {
                    color: Color::from_rgba(0.4, 0.9, 0.5, 0.2),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
            .into()
        } else {
            container(GlassButton::new("Power On Webcam").on_press(Message::ToggleWebcam(true)))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        };

        let footer_action: Element<'static, Message> =
            if self.identified_data.is_some() || self.not_found {
                GlassButton::new("Rescan Feed")
                    .on_press(Message::ResetWebcamSearch)
                    .into()
            } else {
                space().width(0).height(0).into()
            };

        let right_content = column![
            row![
                GlassButton::new("← Back").on_press(Message::GoTo(Page::MainMenu)),
                space().width(Length::Fill),
                GlassInputLabel::new("Webcam Identify").size(32),
            ]
            .align_y(Alignment::Center),
            space().height(40.0),
            if self.is_identifying {
                self.status_view("IDENTIFYING...", Color::from_rgb(0.4, 0.9, 0.5))
            } else if self.not_found {
                self.status_view("NOT IN DATABASE", Color::from_rgb(0.9, 0.4, 0.4))
            } else if let Some(data) = &self.identified_data {
                self.details_view(data)
            } else {
                self.status_view("SCANNING FEED", Color::from_rgba(1.0, 1.0, 1.0, 0.5))
            },
            space().height(Length::Fill),
            footer_action // Using the annotated variable
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

    // status_view and details_view remain the same as previously implemented...
    fn status_view(&self, label: &str, color: Color) -> Element<'static, Message> {
        let label_owned = label.to_string();
        column![
            text(label_owned).size(28).color(color),
            space().height(20.0),
            container(space().height(2.0))
                .width(250.0)
                .style(move |_| container::Style {
                    background: Some(iced::Background::Color(color)),
                    ..Default::default()
                })
        ]
        .align_x(Alignment::Center)
        .into()
    }

    fn details_view(&self, data: &criminal::Model) -> Element<'static, Message> {
        let name = data.name.clone();
        column![
            text("MATCH CONFIRMED")
                .size(24)
                .color(Color::from_rgb(0.4, 0.9, 0.5)),
            space().height(30.0),
            text(name).size(32).color(Color::WHITE),
            space().height(20.0),
            text(format!("Criminal ID: {}", data.criminal_id))
                .size(16)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
            space().height(10.0),
            text(format!("Violation History: {} counts", data.no_of_crimes)).size(18),
        ]
        .align_x(Alignment::Start)
        .into()
    }
}
