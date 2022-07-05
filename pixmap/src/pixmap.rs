use std::collections::HashMap;

/// 8-bit Rgba values in that order.
pub type Rgba8 = (u8, u8, u8, u8);

/// Errors for `pixmap_to_rgba8()` method.
#[derive(Debug)]
pub enum Error {
    /// `pixels` contains a character not registered in palette.
    UnregisteredCharacter,
}

/// Convert XPM-like data into raw rgba8 data. Dimension information
/// is not tracked by this class.
pub fn pixmap_to_rgba8(palette: &HashMap<char, Rgba8>, pixels: &str) -> Result<Vec<u8>, Error> {
    let mut rgba8 = Vec::with_capacity(4 * pixels.chars().count());
    for px in pixels.chars() {
        if let Some(colour) = palette.get(&px) {
            rgba8.append(vec![colour.0, colour.1, colour.2, colour.3].as_mut());
        } else {
            return Err(Error::UnregisteredCharacter);
        }
    }
    Ok(rgba8)
}
