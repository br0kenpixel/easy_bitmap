use easy_bitmap::prelude::BitMap8;

#[test]
fn owned_iter_simple() {
    let mut bitmap = BitMap8::new();

    bitmap.set_nth(0, true).unwrap();
    bitmap.set_nth(1, true).unwrap();
    bitmap.set_nth(2, true).unwrap();
    bitmap.set_nth(3, false).unwrap();
    bitmap.set_nth(4, false).unwrap();
    bitmap.set_nth(5, false).unwrap();
    bitmap.set_nth(6, true).unwrap();
    bitmap.set_nth(7, false).unwrap();

    let mut iterator = bitmap.into_iter();

    assert_eq!(iterator.next(), Some(true));
    assert_eq!(iterator.next(), Some(true));
    assert_eq!(iterator.next(), Some(true));
    assert_eq!(iterator.next(), Some(false));
    assert_eq!(iterator.next(), Some(false));
    assert_eq!(iterator.next(), Some(false));
    assert_eq!(iterator.next(), Some(true));
    assert_eq!(iterator.next(), Some(false));

    assert_eq!(iterator.next(), None);
    assert_eq!(iterator.next(), None);
}

#[test]
#[rustfmt::skip]
fn owned_iter_with_back() {
    let mut bitmap = BitMap8::new();

    bitmap.set_nth(0, true).unwrap();  // <-------------------------------------
    bitmap.set_nth(1, true).unwrap();  // <---------------------------         |
    bitmap.set_nth(2, true).unwrap();  // <-----------------         |         |
    bitmap.set_nth(3, false).unwrap(); // <-----| - false  | - true  | - true  | - true
    bitmap.set_nth(4, false).unwrap(); // <-----| - false  | - false | - false | - false
    bitmap.set_nth(5, false).unwrap(); // <-----------------         |         |
    bitmap.set_nth(6, true).unwrap();  // <---------------------------         |
    bitmap.set_nth(7, false).unwrap(); // <-------------------------------------

    let mut iterator = bitmap.into_iter();

    assert_eq!(iterator.next(), Some(true));
    assert_eq!(iterator.next_back(), Some(false));

    assert_eq!(iterator.next(), Some(true));
    assert_eq!(iterator.next_back(), Some(true));

    assert_eq!(iterator.next(), Some(true));
    assert_eq!(iterator.next_back(), Some(false));

    assert_eq!(iterator.next(), Some(false));
    assert_eq!(iterator.next_back(), Some(false));

    assert_eq!(iterator.next(), None);
    assert_eq!(iterator.next_back(), None);

    assert_eq!(iterator.next(), None);
    assert_eq!(iterator.next_back(), None);
}
