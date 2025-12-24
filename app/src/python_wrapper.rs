// main.rs
use iced::widget::{button, column, container, text, text_input};
use iced::{Center, Element, Fill, Subscription, Task};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;

fn wrapper_main() -> iced::Result {
    iced::application("Python Wrapper", App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .run()
}

struct App {
    output: Vec<String>,
    input_text: String,
    tx: Option<mpsc::UnboundedSender<String>>,
    python_started: bool,
}

#[derive(Debug, Clone)]
enum Message {
    PythonOutput(String),
    InputChanged(String),
    SendInput,
    StartPython,
    PythonReady(mpsc::UnboundedSender<String>),
}

impl Default for App {
    fn default() -> Self {
        Self {
            output: Vec::new(),
            input_text: String::new(),
            tx: None,
            python_started: false,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PythonOutput(line) => {
                self.output.push(line);
                if self.output.len() > 50 {
                    self.output.remove(0);
                }
                Task::none()
            }
            Message::InputChanged(value) => {
                self.input_text = value;
                Task::none()
            }
            Message::SendInput => {
                if let Some(tx) = &self.tx {
                    let input = self.input_text.clone() + "\n";
                    let _ = tx.send(input);
                    self.input_text.clear();
                }
                Task::none()
            }
            Message::StartPython => {
                self.python_started = true;
                Task::none()
            }
            Message::PythonReady(tx) => {
                self.tx = Some(tx);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let output_text = self.output.join("\n");

        let content = column![
            text("Python Process Output").size(24),
            container(text(output_text))
                .padding(10)
                .width(Fill)
                .height(300)
                .style(|_theme: &iced::Theme| {
                    container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(
                            0.1, 0.1, 0.1,
                        ))),
                        border: iced::Border {
                            color: iced::Color::from_rgb(0.3, 0.3, 0.3),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                }),
            text_input("Type input for Python...", &self.input_text)
                .on_input(Message::InputChanged)
                .on_submit(Message::SendInput),
            button("Send to Python").on_press(Message::SendInput),
            if !self.python_started {
                button("Start Python").on_press(Message::StartPython)
            } else {
                button("Python Running")
            }
        ]
        .spacing(10)
        .padding(20)
        .align_x(Center);

        container(content)
            .width(Fill)
            .height(Fill)
            .center(Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.python_started {
            python_subscription()
        } else {
            Subscription::none()
        }
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

fn python_subscription() -> Subscription<Message> {
    struct PythonWorker;

    Subscription::run_with_id(
        std::any::TypeId::of::<PythonWorker>(),
        tokio_stream::wrappers::UnboundedReceiverStream::new({
            let (output_tx, output_rx) = mpsc::unbounded_channel();
            let (stdin_tx, mut stdin_rx) = mpsc::unbounded_channel::<String>();

            tokio::spawn(async move {
                // Start Python process using tokio::process
                let mut child = tokio::process::Command::new("python3")
                    .arg("-u")
                    .arg("rust.py")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to start Python");

                let mut stdin = child.stdin.take().unwrap();
                let stdout = child.stdout.take().unwrap();
                let stderr = child.stderr.take().unwrap();

                // Send the stdin channel back
                let _ = output_tx.send(Message::PythonReady(stdin_tx));

                // Handle stdin writes
                let output_tx_clone = output_tx.clone();
                tokio::spawn(async move {
                    while let Some(input) = stdin_rx.recv().await {
                        if let Err(e) = stdin.write_all(input.as_bytes()).await {
                            let _ = output_tx_clone.send(Message::PythonOutput(format!(
                                "Error writing to stdin: {}",
                                e
                            )));
                            break;
                        }
                        let _ = stdin.flush().await;
                    }
                });

                // Handle stdout
                let output_tx_clone = output_tx.clone();
                tokio::spawn(async move {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        let _ = output_tx_clone.send(Message::PythonOutput(line));
                    }
                });

                // Handle stderr
                tokio::spawn(async move {
                    let reader = BufReader::new(stderr);
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        let _ = output_tx.send(Message::PythonOutput(format!("[ERROR] {}", line)));
                    }
                });
            });

            output_rx
        }),
    )
}
