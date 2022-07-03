use std::collections::HashMap;

/// This represents image data, simular to XPM. Dimensional information is not stored
/// since all tile sprites have the same size. See tests for examples.
pub struct PixMap {
    /// Maps characters to rgba information.
    pub colours: Vec<(char, (u8, u8, u8, u8))>,

    /// String of characters to form image.
    pub pixels: String
}

impl PixMap {
    /// Uncompress into to raw rgba8 information.
    pub fn to_rgba8(&self) -> Vec<u8> {
        let mut colours = HashMap::new();
        for colour in &self.colours {
            colours.insert(colour.0, colour.1);
        }

        let mut rgba8 = Vec::new();
        for colour in self.pixels.chars() {
            let colour = colours[&colour];
            rgba8.append(vec![colour.0, colour.1, colour.2, colour.3].as_mut());
        }
        rgba8
    }
}