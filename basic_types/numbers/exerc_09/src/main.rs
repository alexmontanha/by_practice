fn main() {
    let mut sum = 0;

    for i in -3..-2 {
        println!("{}", i);
        sum += i
    };
    
    assert_eq!(sum, -3);

    for c in 'a'..='z' {
        println!("{}", c as u32);
    }
}
