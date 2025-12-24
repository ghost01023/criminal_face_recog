use app::database::CriminalDB;
use app::pages::*;
use app::Message;
use app::Page;
use iced::{Application, Command, Element, Settings, Theme};
use rfd;

pub struct GlassmorphismApp {
    current_page: Page,
    registry_state: RegistryPage, // The form data lives here
    image_find: ImageFindPage,
    db: Option<std::sync::Arc<CriminalDB>>,
}

impl Application for GlassmorphismApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    fn new(_flags: ()) -> (Self, Command<Message>) {
        let db_url = "mysql://root:@localhost:3306/criminal_recognizer".to_string();

        (
            Self {
                current_page: Page::MainMenu,
                registry_state: RegistryPage::default(),
                image_find: ImageFindPage::default(),
                db: None,
            },
            Command::perform(
                async move {
                    // Map the internal database result to a clean Result<Arc, String>
                    CriminalDB::new(&db_url)
                        .await
                        .map(std::sync::Arc::new)
                        .map_err(|e| e.to_string())
                },
                // FIX: Use a closure instead of passing the Enum variant directly
                |result| Message::DbConnected(result),
            ),
        )
    }
    fn title(&self) -> String {
        String::from("Criminal Recognizer")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GoTo(page) => {
                self.current_page = page;
                Command::none()
            }
            Message::DbConnected(Ok(db_arc)) => {
                self.db = Some(db_arc);
                Command::none()
            }
            Message::OpenFilePicker => Command::perform(
                async {
                    rfd::FileDialog::new()
                        .add_filter("Images", &["jpg", "png", "jpeg"])
                        .pick_files()
                        .unwrap_or_default()
                },
                Message::FilesSelected,
            ),
            Message::FilesSelected(_) => match self.current_page {
                Page::ImageFind => ImageFindPage::update(&mut self.image_find, message),
                _ => self.registry_state.update(message, self.db.clone()),
            },
            // Delegate all other form/registry messages to the registry_state
            _ => self.registry_state.update(message, self.db.clone()),
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Dashboard => {
                // Return a different view here
                iced::widget::text("Welcome to Dashboard").into()
            }
            Page::Registry => {
                // Call the Registry Page's view
                self.registry_state.view()
            }
            Page::MainMenu => MainMenu::view(),
            Page::ImageFind => return self.image_find.view(),
            _ => iced::widget::text("New page").into(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

pub fn main() -> iced::Result {
    GlassmorphismApp::run(Settings::default())
}
