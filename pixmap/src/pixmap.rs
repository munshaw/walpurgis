use std::collections::{HashMap, HashSet};

/// Errors for `PixMap`
#[derive(Debug)]
pub enum Error {
    /// PixMap `pixels` string contains a character unregistered in `colours`.
    UnregisteredPixel
}

/// Represents an rgba colour, in that order.
pub type Rgba = (u8, u8, u8, u8);

/// This represents image data, simular to XPM. Dimensional information is not stored
/// since all tile sprites have the same size. See tests for examples.
pub struct PixMap {
    colours: Vec<(char, Rgba)>,
    pixels: String,
}

impl PixMap {
   /// Create new `PixMap`.
   /// # Arguments
   /// `colours` registers each character with a colour, and `pixels`
   /// are a string of characters, allowing you to draw in ascii.
   pub fn new(colours: Vec<(char, Rgba)>, pixels: &str) -> Result<Self, Error> {
       let mut colour_set = HashSet::new();
       colours.iter().for_each(|c| { colour_set.insert(c.0); });
       if pixels.chars().all(|c| colour_set.contains(&c)) {
           Ok(Self { colours, pixels: pixels.to_string() })
       } else {
           Err(Error::UnregisteredPixel)
       }
   }

    /// Uncompress into to raw rgba8 information.
    pub fn to_rgba8(&self) -> Vec<u8> {
        let mut colour_map = HashMap::new();
        for colour in &self.colours {
            colour_map.insert(colour.0, colour.1);
        }

        let mut rgba8 = Vec::new();
        for pix in self.pixels.chars() {
            let pix_colour = colour_map[&pix];
            rgba8.append(vec![pix_colour.0, pix_colour.1, pix_colour.2, pix_colour.3].as_mut());
        }
        rgba8
    }
}
