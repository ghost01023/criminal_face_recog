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
}

impl Default for VideoFindPage {
    fn default() -> Self {
        Self {
            selected_video: None,
            is_scanning: false,
        }
    }
}

impl VideoFindPage {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FilesSelected(paths) => {
                if let Some(path) = paths.first() {
                    self.selected_video = Some(path.to_string_lossy().to_string());
                    self.is_scanning = true; // Auto-start scanning when video is picked
                }
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
                // Scanning Progress Bar
                container(Space::with_height(Length::Fixed(2.0)))
                    .width(Length::Fixed(240.0))
                    .style(iced::theme::Container::Custom(Box::new(ScanningBarStyle))),
                Space::with_height(40),
                GlassButton::new("Cancel Scan").on_press(Message::GoTo(Page::MainMenu)),
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
