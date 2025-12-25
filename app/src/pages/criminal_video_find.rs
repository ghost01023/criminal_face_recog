use crate::components::GlassButton;
use crate::components::GlassInputLabel;
use crate::components::VideoViewer;
use crate::{Message, Page};

use iced::{
    widget::{column, container, row, text, Space},
    Alignment, Color, Command, Element, Length, Renderer, Theme,
};

pub struct VideoFindPage {
    pub selected_video: Option<String>,
    pub is_scanning: bool,
    pub show_details: bool,
    pub identified_id: Option<String>,
}

impl Default for VideoFindPage {
    fn default() -> Self {
        Self {
            selected_video: None,
            is_scanning: false,
            show_details: false,
            identified_id: None,
        }
    }
}
impl VideoFindPage {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FilesSelected(paths) => {
                if let Some(path) = paths.first() {
                    let path_str = path.to_string_lossy().to_string();
                    self.selected_video = Some(path_str.clone());
                    self.is_scanning = true;
                    self.show_details = false;

                    // Trigger the Python command via the main update loop
                    return Command::perform(async {}, move |_| {
                        println!("PATH OF VIDEO IS {}", path_str);
                        Message::IdentifyCriminalVideo(path_str)
                    });
                }
                Command::none()
            }

            Message::Identity(criminal_id) => {
                self.is_scanning = false;
                self.show_details = true;
                self.identified_id = Some(criminal_id);
                Command::none()
            }

            _ => Command::none(),
        }
    }
    pub fn view(&self) -> Element<Message> {
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

        // --- RIGHT SIDE: 40% Width (Status & Controls) ---
        let right_content: Element<Message, Theme, Renderer> = if self.is_scanning {
            column![
                text("SCANNING VIDEO...")
                    .size(22)
                    .style(iced::theme::Text::Color(Color::from_rgba(
                        0.4, 0.8, 1.0, 1.0
                    ))),
                Space::with_height(10),
                text("Deep recognition in progress")
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
        } else if self.show_details {
            let id_text = self.identified_id.as_deref().unwrap_or("Unknown");
            column![
                text("Target Identified")
                    .size(26)
                    .style(iced::theme::Text::Color(Color::from_rgba(
                        0.4, 0.9, 0.5, 1.0
                    ))),
                Space::with_height(10),
                text(format!("ID: {}", id_text))
                    .size(20)
                    .style(Color::WHITE),
                Space::with_height(30),
                GlassButton::new("New Search").on_press(Message::OpenFilePicker),
                Space::with_height(10),
                GlassButton::new("← Back to Menu").on_press(Message::GoTo(Page::MainMenu)),
            ]
            .align_items(Alignment::Center)
            .into()
        } else {
            column![
                text("Awaiting Video Feed").style(iced::theme::Text::Color(Color::from_rgba(
                    1.0, 1.0, 1.0, 0.3
                ))),
                Space::with_height(20),
                GlassButton::new("← Back to Menu").on_press(Message::GoTo(Page::MainMenu)),
            ]
            .align_items(Alignment::Center)
            .into()
        };

        let right_side = container(right_content)
            .width(Length::FillPortion(40))
            .height(Length::Fill)
            .center_x()
            .center_y();

        row![left_side, right_side].into()
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
