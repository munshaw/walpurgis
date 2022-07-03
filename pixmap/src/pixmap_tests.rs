use crate::pixmap::{Error, PixMap};

#[test]
fn to_rgba8_success_test() {
    let pixmap = PixMap::new(
        vec![('.', (0x00, 0x00, 0x00, 0xff))], "\
        ..\
        .."
    );

    let result;
    match pixmap {
        Ok(r) => result = r.to_rgba8(),
        Err(_) => panic!()
    }

    let expected = vec![0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255];

    assert_eq!(result, expected);
}

#[test]
fn to_rgba8_fail_test() {
    let pixmap = PixMap::new(
        vec![('.', (0x00, 0x00, 0x00, 0xff))], "\
        .!\
        .."
    );

    match pixmap {
        Ok(_) => panic!(),
        Err(_) => ()
    }
}
