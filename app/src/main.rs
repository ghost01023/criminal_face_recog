use app::database::CriminalDB;
use app::pages::*;
use app::python_process::{python_sub, PythonProcess};
use app::Message;
use app::Page;

use iced::{Element, Subscription, Task, Theme};
use std::sync::Arc;

pub struct GlassmorphismApp {
    current_page: Page,
    registry_state: RegistryPage,
    image_find: ImageFindPage,
    video_find: VideoFindPage,
    model_engine: Option<PythonProcess>,
    db: Option<Arc<CriminalDB>>,
}

impl GlassmorphismApp {
    pub fn new() -> (Self, Task<Message>) {
        let db_url = "mysql://root:@localhost:3306/criminal_recognizer".to_string();

        let engine = PythonProcess::spawn(
            "main.py",
            "/home/NEW_VOLUME-d/developer/criminal_face_recog/model_engine",
        )
        .ok();

        let app = Self {
            current_page: Page::MainMenu,
            registry_state: RegistryPage::default(),
            image_find: ImageFindPage::default(),
            video_find: VideoFindPage::default(),
            model_engine: engine,
            db: None,
        };

        let init_task = Task::batch(vec![
            Task::perform(
                async move {
                    CriminalDB::new(&db_url)
                        .await
                        .map(Arc::new)
                        .map_err(|e| e.to_string())
                },
                Message::DbConnected,
            ),
            Task::perform(
                async {
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                },
                |_| Message::InitializePython,
            ),
        ]);

        (app, init_task)
    }

    pub fn title(&self) -> String {
        "Criminal Recognizer".to_string()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitializePython => {
                if let Some(ref mut engine) = self.model_engine {
                    let _ = engine.send("start");
                }
                Task::none()
            }

            Message::PythonOutput(text) => {
                eprintln!("[RUST] Python output: {}", text);

                if text.starts_with("added") {
                    return self.registry_state.update(Message::SaveResult(Ok(0)), None);
                }

                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.get(0) == Some(&"identity") {
                    if let Some(identity_val) = parts.get(1).map(|s| s.to_string()) {
                        if matches!(self.current_page, Page::ImageFind | Page::VideoFind) {
                            return Task::done(Message::Identity(identity_val));
                        }
                    }
                }
                Task::none()
            }

            Message::OpenFilePicker => {
                let filter_name = match self.current_page {
                    Page::VideoFind => "Videos",
                    _ => "Images",
                };
                let extensions = match self.current_page {
                    Page::VideoFind => vec!["mp4", "mkv", "webm", "avi"],
                    _ => vec!["jpg", "png", "jpeg"],
                };

                Task::perform(
                    async move {
                        let file = rfd::AsyncFileDialog::new()
                            .add_filter(filter_name, &extensions)
                            .pick_file()
                            .await;

                        file.map(|handle| vec![handle.path().to_path_buf()])
                            .unwrap_or_default()
                    },
                    Message::FilesSelected,
                )
            }

            Message::GoTo(page) => {
                self.current_page = page;
                self.image_find.selected_image = Vec::new();
                Task::none()
            }

            Message::DbConnected(Ok(db_arc)) => {
                self.db = Some(db_arc);
                Task::none()
            }

            Message::Identity(_) | Message::IdentityDataLoaded(_) | Message::IdentityError(_) => {
                match self.current_page {
                    Page::ImageFind => self.image_find.update(message, self.db.clone()),
                    Page::VideoFind => self.video_find.update(message, self.db.clone()),
                    _ => Task::none(),
                }
            }

            Message::FilesSelected(_) => match self.current_page {
                Page::Registry => self.registry_state.update(message, self.db.clone()),
                Page::ImageFind => self.image_find.update(message, self.db.clone()),
                Page::VideoFind => self.video_find.update(message, self.db.clone()),
                _ => Task::none(),
            },

            _ => match self.current_page {
                Page::ImageFind => self.image_find.update(message, self.db.clone()),
                Page::VideoFind => self.video_find.update(message, self.db.clone()),
                Page::Registry => self.registry_state.update(message, self.db.clone()),
                _ => Task::none(),
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.current_page {
            Page::Dashboard => iced::widget::text("Welcome to Dashboard").into(),
            Page::Registry => self.registry_state.view(),
            Page::MainMenu => MainMenu::view(),
            Page::ImageFind => self.image_find.view(),
            Page::VideoFind => self.video_find.view(),
            _ => iced::widget::text("New page").into(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        python_sub()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

pub fn main() -> iced::Result {
    iced::application(
        GlassmorphismApp::new,
        GlassmorphismApp::update,
        GlassmorphismApp::view,
    )
    .subscription(GlassmorphismApp::subscription)
    .theme(GlassmorphismApp::theme)
    .run()
}
