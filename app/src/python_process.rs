use crate::Message;
use iced::subscription;
use std::{
    io::{BufRead, BufReader, Write},
    process::{ChildStdin, Command, Stdio},
    thread,
};
use tokio::sync::mpsc;

pub struct PythonProcess {
    stdin: ChildStdin,
    // We use Option so we can .take() the receiver out into the subscription
    pub rx: Option<mpsc::Receiver<String>>,
    _child: std::process::Child,
}

/// The Subscription worker
pub fn python_sub(rx: mpsc::Receiver<String>) -> iced::Subscription<Message> {
    // "python_engine" is a unique ID for this subscription instance
    subscription::unfold("python_engine", rx, move |mut rx| async move {
        // This awaits until Python prints a line. No Ticks or loops required.
        match rx.recv().await {
            Some(line) => (Message::PythonOutput(line), rx),
            None => {
                // If the channel closes, we keep the subscription alive but idle
                iced::futures::future::pending().await
            }
        }
    })
}

impl PythonProcess {
    pub fn spawn(script_path: &str, working_dir: &str) -> std::io::Result<Self> {
        let mut child = Command::new("prime-run")
            .arg("python")
            .arg("-u") // Force unbuffered output
            .arg(script_path)
            .current_dir(working_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let stderr = child.stderr.take().expect("Failed to open stderr");

        // Create an async channel with a buffer
        let (tx, rx) = mpsc::channel(100);

        // STDOUT Thread: Moves data from the blocking pipe to the async channel
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        // If this fails, the receiver was dropped
                        if tx.blocking_send(l).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }); // STDERR Thread
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(l) = line {
                    eprintln!("[PY-ERR] {}", l);
                }
            }
        });

        Ok(Self {
            stdin,
            rx: Some(rx),
            _child: child,
        })
    }

    pub fn send(&mut self, message: &str) -> std::io::Result<()> {
        writeln!(self.stdin, "{}", message)?;
        self.stdin.flush()
    }
}
