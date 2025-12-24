use crate::entities::criminal_photo;
use iced::Element;
use iced::widget::image;
use iced::widget::{Column, Image, Scrollable};
use image::load_from_memory;

/// Convert DB image bytes (JPEG/PNG) → iced image handle
fn bytes_to_handle(bytes: &[u8]) -> image::Handle {
    let img = load_from_memory(bytes)
        .expect("Invalid or unsupported image format")
        .to_rgba8();

    let (width, height) = img.dimensions();

    image::Handle::from_rgba(width, height, img.into_raw())
}

/// Render all criminal photos in a scrollable view
pub fn render_criminal_photos(photos: &[criminal_photo::Model]) -> Element<'static, ()> {
    let column = photos.iter().fold(Column::new().spacing(12), |col, photo| {
        let handle = bytes_to_handle(&photo.photo);
        col.push(Image::new(handle).width(200))
    });

    Scrollable::new(column).into()
}
