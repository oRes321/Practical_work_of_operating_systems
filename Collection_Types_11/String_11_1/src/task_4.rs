#[test]
fn main() {
    let s = String::from("hello, 世界");

    let slice1 = &s[0..1]; // 'h' занимает 1 байт
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10]; // '世' занимает 3 байта и начинается с 7-го байта
    assert_eq!(slice2, "世");

    // Перебираем символы с индексами
    for (i, c) in s.char_indices() {
        if i == 7 {
            assert_eq!(c, '世');
        }
    }

    println!("Success!");
}
