use crate::pixmap::{pixmap_to_rgba8, Error};
use std::collections::HashMap;

#[test]
fn pixmap_to_rgba8_ok() {
    let mut palette = HashMap::new();
    palette.insert('.', (0x00, 0x00, 0x00, 0x00));
    palette.insert('x', (0x01, 0x01, 0x01, 0x01));
    let pixels = "\
    .x\
    .x";

    let expected = vec![0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1];
    let actual = pixmap_to_rgba8(&palette, pixels).unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn pixmap_to_rgba8_err() {
    let mut palette = HashMap::new();
    palette.insert('.', (0x00, 0x00, 0x00, 0x00));
    palette.insert('x', (0x01, 0x01, 0x01, 0x01));
    let pixels = "\
    .!\
    .x";

    if let Err(Error::UnregisteredCharacter) = pixmap_to_rgba8(&palette, pixels) {
    } else {
        panic!()
    }
}
