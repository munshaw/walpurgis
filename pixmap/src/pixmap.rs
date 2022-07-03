use std::collections::HashMap;

/// This represents image data, simular to XPM. Dimensional information is not stored
/// since all tile sprites have the same size. See tests for examples.
pub struct PixMap {
    /// Maps characters to rgba information.
    pub colours: HashMap<char, (u8, u8, u8, u8)>,

    /// String of characters to form image.
    pub pixels: String
}

impl PixMap {
    /// Uncompress into to raw rgba8 information.
    pub fn to_rgba8(&self) -> Vec<u8> {
        let mut rgba8 = Vec::new();
        for c in self.pixels.chars() {
            let r = self.colours[&c];
            rgba8.append(vec![r.0, r.1, r.2, r.3].as_mut());
        }
        rgba8
    }
}