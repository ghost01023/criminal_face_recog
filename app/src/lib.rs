pub mod components;
pub mod database;
pub mod entities;
pub mod pages;
pub mod python_process;
pub mod webcam_task;

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
    WebcamFind,
    SignIn,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    // --- Page Navigation ---
    GoTo(Page),
    IdentityDataLoadedWithPhotos(
        entities::criminal::Model,
        Vec<entities::criminal_photo::Model>,
    ),
    TickWebcam,
    CaptureWebcamFrame,
    WebcamFrameCaptured(String), // The path to the temp file
    ToggleWebcam(bool),
    ResetWebcamSearch,
    ResetForm,
    DatabaseSaved(u32, Vec<String>),
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

    // --- PythonProcess model_engine Events ---
    InitializePython,
    IdentifyCriminalImage(String),
    IdentifyCriminalVideo(String),
    PythonInput(String),
    PythonOutput(String),
    Identity(String),
    IdentityDataLoaded(entities::criminal::Model), // Success from DB
    IdentityError(String),
}
