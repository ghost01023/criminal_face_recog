use iced::widget::Text;
use iced::{Application, Element, Settings, executor};
use sea_orm::Database;
use tokio::process::Command;

use app::db::CriminalDb;
use app::entities::criminal_photo;
use app::helper::photo_display::render_criminal_photos;

fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    photos: Option<Vec<criminal_photo::Model>>,
}

#[derive(Debug, Clone)]
enum Message {
    PhotosLoaded(Vec<criminal_photo::Model>),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        (
            Self { photos: None },
            Command::perform(load_photos(), Message::PhotosLoaded),
        )
    }

    fn title(&self) -> String {
        "Criminal Photo Viewer".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PhotosLoaded(photos) => {
                self.photos = Some(photos);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match &self.photos {
            Some(photos) => {
                // map () messages to Message (none emitted)
                render_criminal_photos(photos).map(|_| unreachable!())
            }
            None => Text::new("Loading photos...").into(),
        }
    }
}

/// Async DB loader
async fn load_photos() -> Vec<criminal_photo::Model> {
    let db = Database::connect("mysql://user:pass@localhost/dbname")
        .await
        .expect("DB connection failed");

    let criminal_db = CriminalDb::new(db);

    criminal_db
        .get_criminal_photos(1) // example criminal_id
        .await
        .unwrap_or_default()
}
