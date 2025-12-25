use crate::Message;
use iced::futures::stream;
use iced::Subscription;

use std::sync::{Arc, Mutex as StdMutex, OnceLock};
use std::{
    io::{BufRead, BufReader, Write},
    process::{ChildStdin, Command, Stdio},
    thread,
};

use tokio::sync::{mpsc, Mutex as TokioMutex};

static PYTHON_RX: OnceLock<Arc<TokioMutex<mpsc::Receiver<String>>>> = OnceLock::new();

pub struct PythonProcess {
    stdin: Arc<StdMutex<ChildStdin>>,
    _child: std::process::Child,
}

pub fn python_sub() -> Subscription<Message> {
    Subscription::run(python_stream)
}
fn python_stream() -> impl futures::Stream<Item = Message> + Send + 'static {
    let rx = PYTHON_RX.get().expect("Python RX not initialized").clone();

    stream::unfold(rx, |rx| async move {
        // scope the lock tightly
        let next = {
            let mut guard = rx.lock().await;
            guard.recv().await
        }; // <-- guard dropped HERE

        match next {
            Some(line) => Some((Message::PythonOutput(line), rx)),
            None => None,
        }
    })
}

impl PythonProcess {
    pub fn spawn(script_path: &str, working_dir: &str) -> std::io::Result<Self> {
        let mut child = Command::new("prime-run")
            .arg("python")
            .arg("-u")
            .arg(script_path)
            .current_dir(working_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let (tx, rx) = mpsc::channel::<String>(100);
        let rx = Arc::new(TokioMutex::new(rx));
        let _ = PYTHON_RX.set(rx.clone());

        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                let _ = tx.blocking_send(line);
            }
        });

        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                eprintln!("[PYTHON-ERR] {}", line);
            }
        });

        Ok(Self {
            stdin: Arc::new(StdMutex::new(stdin)),
            _child: child,
        })
    }

    pub fn send(&self, message: &str) -> std::io::Result<()> {
        let mut stdin = self.stdin.lock().unwrap();
        writeln!(&mut *stdin, "{}", message)?;
        stdin.flush()
    }
}
