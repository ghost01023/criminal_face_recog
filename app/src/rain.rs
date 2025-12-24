/*use app::db::CriminalDB;
use app::entities::criminal_photo;
use iced::widget::{container, text};
use iced::{Element, Task};
use sea_orm::Database;

fn main() -> iced::Result {
    iced::application("Criminal Photo Viewer", update, view).run_with(|| {
        (
            App {
                photos: None,
                status: Some("Attempting to load...".to_string()),
            },
            Task::perform(load_photos(), Message::PhotosLoaded),
        )
    })
}

struct App {
    photos: Option<Vec<criminal_photo::Model>>,
    status: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            photos: None,
            status: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    PhotosLoaded(Result<Vec<criminal_photo::Model>, String>),
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::PhotosLoaded(res) => match res {
            Ok(photos) => {
                if photos.is_empty() {
                    app.photos = None;
                    app.status = Some("No photos were found".to_string());
                } else {
                    app.photos = Some(photos);
                    app.status = Some("Photos loaded".to_string());
                }
            }
            Err(err) => {
                app.photos = None;
                app.status = Some(err);
            }
        },
    }
    Task::none()
}

fn view(app: &App) -> Element<Message> {
    println!("Status changed");
    println!("{}", app.status.as_deref().unwrap_or(""));
    match &app.status {
        Some(status) => container(text(format!("{}", status.to_uppercase())))
            .padding(20)
            .into(),
        None => container(text("No Photos present")).padding(20).into(),
    }
}

/// Async DB loader
async fn load_photos() -> Result<Vec<criminal_photo::Model>, String> {
    let criminal_db = CriminalDB::new("mysql://root:@localhost/criminal_recognizer")
        .await
        .map_err(|e| format!("DB connection failed: {}", e))?;

    criminal_db
        .get_criminal_photos(1) // example criminal_id
        .await
        .map_err(|e| format!("Failed to fetch photos: {}", e))
}*/

use iced::{
    widget::{column, container, Space},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

// Import your components
mod components; // Assuming they are in a components/ folder with a mod.rs
use components::button::GlassButton;
use components::image_viewer::GlassImageViewer;
use components::input_label::GlassInputLabel;
use components::text_input::GlassTextInput;

pub fn smain() -> iced::Result {
    GlassmorphismApp::run(Settings::default())
}

struct GlassmorphismApp {
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonPressed,
    NextImage,
    PrevImage,
}

impl Application for GlassmorphismApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            GlassmorphismApp {
                input_value: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Glass Morphism UI")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::ButtonPressed => {
                println!("Button pressed! Input: {}", self.input_value);
            }
            Message::NextImage => {
                println!("Next image requested.");
            }

            Message::PrevImage => {
                println!("Prev image requested.");
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Main Title with Neon Green
        let title = GlassInputLabel::new("Criminal Face Recognizer")
            .size(48)
            .color([0.4, 0.9, 0.5]);

        // Subtitle (Default color)
        let subtitle = GlassInputLabel::new("using FaceShift")
            .size(32)
            .color([0.9, 0.9, 0.9]);

        // Description with muted grey
        let description = GlassInputLabel::new("Isolated Objects & Editable Colors")
            .size(14)
            .color([0.7, 0.7, 0.7]);

        // Styled Text Input
        let input =
            GlassTextInput::new("Enter text...", &self.input_value).on_input(Message::InputChanged);

        // Styled Button
        let button = GlassButton::new("Submit").on_press(Message::ButtonPressed);
        let k = vec!["salman.jpg".to_string(), "salman_2.jpg".to_string()];
        let mut h: usize = 0;

        let image_viewer = GlassImageViewer::new(k, h).view(
            Message::NextImage, // Passing the "Next" message
            Message::PrevImage, // Passing the "Prev" message
        );
        // Glass Image Placeholder

        let content = column![
            Space::with_height(50),
            title,
            subtitle,
            Space::with_height(10),
            description,
            Space::with_height(40),
            // Wrapping input in a specific width for better "Glass" look
            container(input).width(Length::Fixed(400.0)),
            Space::with_height(20),
            button,
            Space::with_height(40),
            image_viewer
        ]
        .padding(40)
        .align_items(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .style(iced::theme::Container::default()) // Keeps the Dark Theme background
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
