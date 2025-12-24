// main.rs
use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
};

pub struct PythonProcess {
    stdin: ChildStdin,
    rx: Receiver<String>,
}

impl PythonProcess {
    pub fn new(mut child: Child) -> Self {
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let (tx, rx) = mpsc::channel();

        // ---- stdout reader thread ----
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = tx.send(line);
                }
            }
        });

        Self { stdin, rx }
    }

    pub fn send_message(&mut self, msg: &str) {
        let _ = writeln!(self.stdin, "{msg}");
        let _ = self.stdin.flush();
    }

    pub fn receive_message(&self) -> Option<String> {
        self.rx.try_recv().ok()
    }
}

fn main() -> std::io::Result<()> {
    let mut child = Command::new("prime-run")
        .arg("python")
        .arg("/home/NEW_VOLUME-d/developer/criminal_face_recog/model_engine/main.py")
        .current_dir("/home/NEW_VOLUME-d/developer/criminal_face_recog/model_engine")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // recommended
        .spawn()?;

    // ─────────────────────────────
    // STDOUT reader thread
    // ─────────────────────────────
    let stdout = child.stdout.take().unwrap();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(l) => println!("[PY] {}", l),
                Err(e) => {
                    eprintln!("Python stdout read error: {}", e);
                    break;
                }
            }
        }
    });

    // ─────────────────────────────
    // STDERR reader thread (VERY IMPORTANT)
    // ─────────────────────────────
    let stderr = child.stderr.take().unwrap();
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(l) => eprintln!("[PY-ERR] {}", l),
                Err(_) => break,
            }
        }
    });

    // ─────────────────────────────
    // Write to Python stdin
    // ─────────────────────────────
    let mut stdin = child.stdin.take().unwrap();

    loop {
        let mut msg = String::new();
        std::io::stdin().read_line(&mut msg)?;

        let msg = msg.trim();
        if msg.is_empty() {
            continue;
        }

        writeln!(stdin, "{}", msg)?;
        stdin.flush()?;
    }
}
