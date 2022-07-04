use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Errors for `PixMap`
#[derive(Debug)]
pub enum Error {
    /// PixMap `pixels` string contains a character unregistered in `palette`.
    UnregisteredPixel,

    /// PixMap `palette` contains a character thats registered twice.
    DuplicateCharacter,
}

/// Represents an rgba colour, in that order.
pub type Rgba = (u8, u8, u8, u8);

/// This represents image data, simular to XPM. Dimensional information is not stored
/// since all tile sprites have the same size. See tests for examples.
#[derive(Debug)]
pub struct PixMap {
    palette: Rc<Vec<(char, Rgba)>>,
    pixels: String,
}

impl PixMap {
    /// Create new `PixMap`.
    /// # Arguments
    /// `palette` registers each character with a colour, and `pixels`
    /// are a string of characters, allowing you to draw in ascii. Dimensional
    /// information is not tracked.
    pub fn new(palette: Rc<Vec<(char, Rgba)>>, pixels: &str) -> Result<Self, Error> {
        // Check if all characters in pixels is registered in the palette.
        let mut colour_set = HashSet::new();
        if !palette.iter().all(|c| colour_set.insert(c.0)) {
            return Err(Error::DuplicateCharacter);
        }
        if !pixels.chars().all(|c| colour_set.contains(&c)) {
            return Err(Error::UnregisteredPixel);
        }

        Ok(Self {
            palette: palette,
            pixels: pixels.to_string(),
        })
    }

    /// Uncompress into to raw rgba8 information.
    pub fn to_rgba8(&self) -> Vec<u8> {
        let mut colour_map = HashMap::new();
        for colour in self.palette.iter() {
            colour_map.insert(colour.0, colour.1);
        }

        let mut rgba8 = Vec::with_capacity(4 * self.pixels.chars().count());
        for pix in self.pixels.chars() {
            let pix_colour = colour_map[&pix];
            rgba8.append(vec![pix_colour.0, pix_colour.1, pix_colour.2, pix_colour.3].as_mut());
        }
        rgba8
    }
}
