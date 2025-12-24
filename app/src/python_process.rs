use crate::Message;
use iced::futures;
use iced::subscription;
use std::{
    io::{BufRead, BufReader, Write},
    process::{ChildStdin, Command, Stdio},
    sync::{Arc, Mutex as StdMutex},
    thread,
};
use tokio::sync::{mpsc, Mutex as TokioMutex};

pub struct PythonProcess {
    stdin: Arc<StdMutex<ChildStdin>>,
    rx: Arc<TokioMutex<mpsc::Receiver<String>>>,
    _child: std::process::Child,
}

pub fn python_sub(rx: Arc<TokioMutex<mpsc::Receiver<String>>>) -> iced::Subscription<Message> {
    subscription::unfold("python_engine", rx, move |rx| async move {
        let mut rx_guard = rx.lock().await;
        match rx_guard.recv().await {
            Some(line) => {
                drop(rx_guard); // Release lock before returning
                (Message::PythonOutput(line), rx)
            }
            None => {
                drop(rx_guard);
                futures::future::pending().await
            }
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

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let stderr = child.stderr.take().expect("Failed to open stderr");

        let (tx, rx) = mpsc::channel(100);

        // STDOUT Thread
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        eprintln!("[PY-OUT-RAW] {}", l);
                        if tx.blocking_send(l).is_err() {
                            eprintln!("[PY-OUT] Channel closed");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("[PY-OUT-ERR] {}", e);
                        break;
                    }
                }
            }
            eprintln!("[PY-OUT] Thread exiting");
        });

        // STDERR Thread
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(l) = line {
                    eprintln!("[PY-ERR] {}", l);
                }
            }
        });

        Ok(Self {
            stdin: Arc::new(StdMutex::new(stdin)),
            rx: Arc::new(TokioMutex::new(rx)),
            _child: child,
        })
    }

    pub fn send(&self, message: &str) -> std::io::Result<()> {
        let mut stdin = self.stdin.lock().unwrap();
        writeln!(&mut *stdin, "{}", message)?;
        stdin.flush()
    }

    pub fn get_rx(&self) -> Arc<TokioMutex<mpsc::Receiver<String>>> {
        Arc::clone(&self.rx)
    }
}
