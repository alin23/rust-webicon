extern crate mime;

use mime::Mime;
use image;

// XXX: Move into Piston?
pub trait AsImageFormat {
    fn parse_image_format(&self) -> Option<(Mime, image::ImageFormat)>;
}

impl AsImageFormat for Mime {
    fn parse_image_format(&self) -> Option<(Mime, image::ImageFormat)> {
        Some(match (self.type_(), self.subtype()) {
            (mime::IMAGE, mime::PNG) => (self.clone(), image::ImageFormat::PNG),
            (mime::IMAGE, mime::JPEG) => (self.clone(), image::ImageFormat::JPEG),
            (mime::IMAGE, mime::GIF) => (self.clone(), image::ImageFormat::GIF),
            (mime::IMAGE, s) if s == "x-icon" || s == "vnd.microsoft.icon" => (
                "image/x-icon".parse::<mime::Mime>().unwrap(),
                image::ImageFormat::ICO,
            ),
            _ => return None
        })
    }
}
