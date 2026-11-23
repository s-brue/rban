use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "ansi/"]
pub struct Ansi;

#[derive(RustEmbed)]
#[folder = "ascii/"]
pub struct Ascii;

#[derive(RustEmbed)]
#[folder = "."]
#[include = "fonts.conf"]
pub struct Conf;

/// Read a file from the ansi/ bundle.
pub fn ansi_file(name: &str) -> Option<Vec<u8>> {
    Ansi::get(name).map(|f| f.data.into_owned())
}

/// Read a file from the ascii/ bundle.
pub fn ascii_file(name: &str) -> Option<Vec<u8>> {
    Ascii::get(name).map(|f| f.data.into_owned())
}

/// Iterate over all filenames in the ascii/ bundle.
pub fn ascii_files() -> impl Iterator<Item = std::borrow::Cow<'static, str>> {
    Ascii::iter()
}

/// Bundled fonts.conf content.
pub fn bundled_conf() -> Option<String> {
    Conf::get("fonts.conf")
        .and_then(|f| String::from_utf8(f.data.into_owned()).ok())
}
