use mime::Mime;
use mime_guess::get_mime_extensions;

pub fn get_extension_from_mime(mime_type: &str) -> Option<&'static str> {
    let mime: Mime = mime_type.parse().ok()?;

    get_mime_extensions(&mime).and_then(|exts| exts.first().copied())
}
