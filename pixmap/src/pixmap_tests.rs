use crate::pixmap::PixMap;
use std::rc::Rc;

#[test]
fn to_rgba8_success_test() {
    let palette = Rc::new(vec![('.', (0x00, 0x00, 0x00, 0xff))]);

    let pixmap = PixMap::new(
        palette,
        "\
        ..\
        ..",
    );

    let result = pixmap.unwrap().to_rgba8();

    let expected = vec![0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255];

    assert_eq!(result, expected);
}

#[test]
fn to_rgba8_fail_test() {
    let palette = Rc::new(vec![('.', (0x00, 0x00, 0x00, 0xff))]);
    let pixmap = PixMap::new(
        palette,
        "\
        .!\
        ..",
    );

    if let Ok(_) = pixmap {
        panic!()
    }
}
