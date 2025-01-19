use mime::Mime;
use mime_guess::get_mime_extensions;

const BANNED_EXTENSIONS: [&str; 2] = ["jfif", "jpe"];

pub fn get_extension_from_mime(mime_type: &str) -> Option<&'static str> {
    let mime: Mime = mime_type.parse().ok()?;

    get_mime_extensions(&mime).and_then(|exts| {
        let mut filtered_exts = exts.iter().filter(|ext| !BANNED_EXTENSIONS.contains(&ext));

        filtered_exts.next().cloned()
    })
}
