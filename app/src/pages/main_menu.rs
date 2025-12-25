use crate::components::GlassButton;
use crate::components::GlassInputLabel;

use crate::{Message, Page};
use iced::{
    widget::{column, container, text},
    Alignment, Element, Length,
};

pub struct MainMenu;

impl MainMenu {
    pub fn view<'a>() -> Element<'a, Message> {
        let title = GlassInputLabel::new("CRIMINAL INTELLIGENCE SYSTEM").size(32);

        let subtitle = text("Select Operation Module").size(16);

        // Create navigation buttons
        let registry_btn =
            GlassButton::new("1. Criminal Registry").on_press(Message::GoTo(Page::Registry));

        let image_find_btn = GlassButton::new("2. Image Search / Recognition")
            .on_press(Message::GoTo(Page::ImageFind));

        let video_find_btn = GlassButton::new("3. Video Analytics / Tracking")
            .on_press(Message::GoTo(Page::VideoFind));

        let webcam_find_btn =
            GlassButton::new("4. Webcam Search").on_press(Message::GoTo(Page::WebcamFind));
        let sign_in_btn =
            GlassButton::new("4. Operator Sign-In / Logs").on_press(Message::GoTo(Page::SignIn));

        let menu_items = column![
            registry_btn,
            image_find_btn,
            video_find_btn,
            webcam_find_btn,
            sign_in_btn
        ]
        .spacing(20)
        .width(Length::Fixed(400.0));

        let content = column![title, subtitle, menu_items,]
            .align_x(Alignment::Center)
            .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
