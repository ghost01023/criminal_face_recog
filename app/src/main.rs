use app::database::CriminalDB;
use app::pages::criminal_registry::RegistryPage;
use app::Message;

use app::Page;
use iced::{Application, Command, Element, Settings, Theme};
use rfd;

pub struct GlassmorphismApp {
    current_page: Page,
    registry_state: RegistryPage, // The form data lives here
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
                current_page: Page::Registry,
                registry_state: RegistryPage::default(),
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
            Message::GoTo(page) => self.current_page = page,

            // Route NameChanged to the Registry state
            Message::NameChanged(s) => {
                self.registry_state.name = s;
                self.registry_state.name_error = false;
            }

            Message::NextImage => {
                if !self.registry_state.selected_images.is_empty() {
                    // Using modulo (%) allows the gallery to loop back to the start
                    self.registry_state.current_img_idx = (self.registry_state.current_img_idx + 1)
                        % self.registry_state.selected_images.len();
                }
            }
            Message::PrevImage => {
                if !self.registry_state.selected_images.is_empty() {
                    // Check for zero to wrap around to the end of the list
                    if self.registry_state.current_img_idx == 0 {
                        self.registry_state.current_img_idx =
                            self.registry_state.selected_images.len() - 1;
                    } else {
                        self.registry_state.current_img_idx -= 1;
                    }
                }
            }
            Message::DbConnected(Ok(db_arc)) => {
                self.db = Some(db_arc);
                println!("Connected to Database.");
            }
            Message::DbConnected(Err(e)) => {
                eprintln!("Database Connection Error: {}", e);
            }
            Message::SubmitForm => {
                if self.registry_state.name.trim().is_empty() {
                    self.registry_state.name_error = true;
                    return Command::none();
                }

                // Check if DB is ready
                let Some(db) = self.db.clone() else {
                    eprintln!("Database not connected yet.");
                    return Command::none();
                };

                // Prepare data for the async block
                let name = self.registry_state.name.clone();
                let f_name = if self.registry_state.fathers_name.is_empty() {
                    None
                } else {
                    Some(self.registry_state.fathers_name.clone())
                };
                let loc = if self.registry_state.arrested_location.is_empty() {
                    None
                } else {
                    Some(self.registry_state.arrested_location.clone())
                };
                let crimes = self.registry_state.no_of_crimes.parse::<u32>().unwrap_or(1);
                let photo_paths = self.registry_state.selected_images.clone();

                return Command::perform(
                    async move {
                        // 1. Add Criminal and get ID
                        let criminal_id = db
                            .add_criminal(name, f_name, loc, crimes)
                            .await
                            .map_err(|e| e.to_string())?;

                        // 2. Add each photo
                        for path in photo_paths {
                            if let Ok(bytes) = std::fs::read(path) {
                                db.add_criminal_photo(criminal_id, bytes)
                                    .await
                                    .map_err(|e| e.to_string())?;
                            }
                        }
                        Ok(criminal_id)
                    },
                    Message::SaveResult,
                );
            }

            Message::SaveResult(Ok(id)) => {
                println!("Criminal successfully saved with ID: {}", id);
                // Optionally clear the form here
                self.registry_state = RegistryPage::default();
            }
            Message::SaveResult(Err(e)) => {
                eprintln!("Failed to save criminal: {}", e);
            }

            // RFD File Picker handling
            Message::OpenFilePicker => {
                return Command::perform(
                    async {
                        rfd::FileDialog::new()
                            .add_filter("Images", &["jpg", "png", "jpeg"])
                            .pick_files()
                            .unwrap_or_default()
                    },
                    Message::FilesSelected,
                );
            }
            Message::FilesSelected(paths) => {
                self.registry_state.selected_images = paths
                    .into_iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();
                self.registry_state.current_img_idx = 0;
            }
            _ => {}
        }
        Command::none()
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
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

pub fn main() -> iced::Result {
    GlassmorphismApp::run(Settings::default())
}
