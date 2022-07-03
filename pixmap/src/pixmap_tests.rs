use std::collections::HashMap;
use crate::pixmap::PixMap;

#[test]
fn decode_black_test() {
    let pixmap = PixMap {
        colours: HashMap::from([('.', (0x00, 0x00, 0x00, 0xff))]),
        pixels: "....".to_string()
    };

    let result = pixmap.to_rgba8();

    let expected = vec![0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255];

    assert_eq!(result, expected);
}