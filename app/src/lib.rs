pub mod components;
pub mod database;
pub mod entities;
pub mod pages;

use crate::database::CriminalDB;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    MainMenu,
    Registry,
    ImageFind,
    VideoFind,
    SignIn,
}

#[derive(Debug, Clone)]
pub enum Message {
    // --- Page Navigation ---
    GoTo(Page),

    // --- Registry Form Inputs ---
    NameChanged(String),
    FathersNameChanged(String),
    CrimesCountChanged(String),
    LocationChanged(String),

    // --- Image Gallery Logic ---
    OpenFilePicker,
    FilesSelected(Vec<PathBuf>),
    NextImage,
    PrevImage,

    // --- Database Operations ---
    SubmitForm,
    DbConnected(Result<Arc<CriminalDB>, String>),
    SaveResult(Result<u32, String>),
}
