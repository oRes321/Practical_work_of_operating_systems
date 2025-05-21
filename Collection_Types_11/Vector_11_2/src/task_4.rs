
// FIX the error and IMPLEMENT the code
#[test]
fn main() {
    let mut v = Vec::from([1, 2, 3]);

    for i in  0..3 {
        println!("{:?}", v[i])
    }
    v.remove(0);


    for i in 4..=6 {
        v.push(i);
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}