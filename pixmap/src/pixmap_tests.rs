use crate::pixmap::PixMap;

#[test]
fn to_rgba8_success_test() {
    let pixmap = PixMap::new(
        vec![('.', (0x00, 0x00, 0x00, 0xff))],
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
    let pixmap = PixMap::new(
        vec![('.', (0x00, 0x00, 0x00, 0xff))],
        "\
        .!\
        ..",
    );

    if let Ok(_) = pixmap {
        panic!()
    }
}
