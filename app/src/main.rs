use app::database::CriminalDB;
use app::pages::*;
use app::python_process::python_sub;
use app::python_process::PythonProcess;
use app::Message;
use app::Page;
use iced::{Application, Command, Element, Settings, Theme};
use rfd;
use std::cell::RefCell;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex as TokioMutex;
pub struct GlassmorphismApp {
    current_page: Page,
    registry_state: RegistryPage, // The form data lives here
    image_find: ImageFindPage,
    video_find: VideoFindPage,
    model_engine: Option<PythonProcess>,
    python_rx: Option<Arc<TokioMutex<mpsc::Receiver<String>>>>,
    db: Option<std::sync::Arc<CriminalDB>>,
}

impl Application for GlassmorphismApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn subscription(&self) -> iced::Subscription<Message> {
        if let Some(ref rx) = self.python_rx {
            eprintln!("[RUST] Subscription active");
            python_sub(Arc::clone(rx))
        } else {
            eprintln!("[RUST] No subscription");
            iced::Subscription::none()
        }
    }
    fn new(_flags: ()) -> (Self, Command<Message>) {
        let db_url = "mysql://root:@localhost:3306/criminal_recognizer".to_string();

        let engine = PythonProcess::spawn(
            "main.py",
            "/home/NEW_VOLUME-d/developer/criminal_face_recog/model_engine",
        )
        .ok();

        let python_rx = engine.as_ref().map(|e| e.get_rx());

        (
            Self {
                current_page: Page::MainMenu,
                registry_state: RegistryPage::default(),
                image_find: ImageFindPage::default(),
                video_find: VideoFindPage::default(),
                model_engine: engine,
                python_rx,
                db: None,
            },
            Command::batch(vec![
                Command::perform(
                    async move {
                        CriminalDB::new(&db_url)
                            .await
                            .map(std::sync::Arc::new)
                            .map_err(|e| e.to_string())
                    },
                    |result| Message::DbConnected(result),
                ),
                Command::perform(
                    async {
                        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                    },
                    |_| Message::InitializePython,
                ),
            ]),
        )
    }
    fn title(&self) -> String {
        String::from("Criminal Recognizer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InitializePython => {
                eprintln!("[RUST] Sending start command to Python");
                if let Some(ref mut engine) = self.model_engine {
                    match engine.send("start") {
                        Ok(_) => eprintln!("[RUST] Start command sent successfully"),
                        Err(e) => eprintln!("[RUST] Failed to send start: {}", e),
                    }
                } else {
                    eprintln!("[RUST] No engine available!");
                }
                Command::none()
            }
            Message::PythonOutput(text) => {
                eprintln!("[RUST] Received from Python: {}", text);
                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.len() > 0 && parts[0] == "identity" {
                    println!("IDENTITY REVEALED");
                }
                Command::none()
            }
            Message::GoTo(page) => {
                self.current_page = page;
                Command::none()
            }
            Message::DbConnected(Ok(db_arc)) => {
                self.db = Some(db_arc);
                Command::none()
            }
            Message::OpenFilePicker => match self.current_page {
                Page::VideoFind => Command::perform(
                    async {
                        rfd::FileDialog::new()
                            .add_filter("Videos", &["mp4", "mkv", "webm", "avi"])
                            .pick_files()
                            .unwrap_or_default()
                    },
                    Message::FilesSelected,
                ),
                Page::ImageFind => Command::perform(
                    async {
                        rfd::FileDialog::new()
                            .add_filter("Images", &["jpg", "png", "jpeg"])
                            .pick_files()
                            .unwrap_or_default()
                    },
                    Message::FilesSelected,
                ),
                Page::Registry => Command::perform(
                    async {
                        rfd::FileDialog::new()
                            .add_filter("Images", &["jpg", "png", "jpeg"])
                            .pick_files()
                            .unwrap_or_default()
                    },
                    Message::FilesSelected,
                ),
                _ => self.registry_state.update(message, self.db.clone()),
            },
            Message::FilesSelected(_) => match self.current_page {
                Page::ImageFind => ImageFindPage::update(&mut self.image_find, message),
                Page::VideoFind => VideoFindPage::update(&mut self.video_find, message),
                _ => self.registry_state.update(message, self.db.clone()),
            },
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
            Page::ImageFind => self.image_find.view(),
            Page::VideoFind => self.video_find.view(),
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
