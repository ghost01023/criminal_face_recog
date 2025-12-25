use crate::components::GlassButton;
use crate::components::GlassImageViewer;
use crate::components::GlassInputLabel;
use crate::{Message, Page};

use iced::{
    widget::{column, container, row, text, Space},
    Alignment, Color, Command, Element, Length, Renderer, Theme,
};

pub struct ImageFindPage {
    pub selected_image: Vec<String>,
    pub is_identifying: bool,
    pub show_details: bool,
}

impl Default for ImageFindPage {
    fn default() -> Self {
        Self {
            selected_image: Vec::new(),
            is_identifying: false,
            show_details: false,
        }
    }
}

impl ImageFindPage {
    // This method now takes &self, allowing it to be called from the app state
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FilesSelected(paths) => {
                // 1. Grab the first path as a String before the vector is moved
                let first_path = paths
                    .get(0)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();

                // 2. Now process the full vector (consuming it)
                self.selected_image = paths
                    .into_iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();

                println!("Loading image now: {}", first_path);

                // 3. Return the command using the string we extracted
                return Command::perform(async {}, move |_| {
                    Message::IdentifyCriminalImage(first_path)
                });
            }

            Message::Identity(criminal_id) => {
                self.is_identifying = false;
                println!("SOMEONE HAS BEEN IDENTIFIED");
                self.show_details = true;
                println!("ID IS: {}", criminal_id);
                return Command::none();
            }

            _ => {
                println!("Inconsequential message");
                return Command::none();
            }
        }
    }
    pub fn view(&self) -> Element<Message> {
        // --- LEFT SIDE: 60% Width ---
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

        // --- RIGHT SIDE: 40% Width ---
        let right_content: Element<Message, Theme, Renderer> =
            if self.is_identifying {
                column![
                    text("Identifying criminal...")
                        .size(22)
                        .style(iced::theme::Text::Color(Color::from_rgba(
                            0.4, 0.9, 0.5, 1.0
                        ))),
                    Space::with_height(20),
                    container(Space::with_height(Length::Fixed(2.0)))
                        .width(Length::Fixed(200.0))
                        .style(iced::theme::Container::Custom(Box::new(LoaderBarStyle))),
                    Space::with_height(40),
                    GlassButton::new("← Back to Menu").on_press(Message::GoTo(Page::MainMenu)),
                ]
                .align_items(Alignment::Center)
                .into()
            } else if self.show_details {
                column![text("Identified").size(30).style(iced::theme::Text::Color(
                    Color::from_rgba(0.4, 0.9, 0.5, 1.0)
                )),]
                .align_items(Alignment::Center)
                .into()
            } else {
                column![
                    text("Awaiting Image Input").style(iced::theme::Text::Color(Color::from_rgba(
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
