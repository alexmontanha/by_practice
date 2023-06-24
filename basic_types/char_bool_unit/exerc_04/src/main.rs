fn main() {
    let f = true;
    let t = true || false;

    assert_eq!(f, t);

    println!("Success!");
}
