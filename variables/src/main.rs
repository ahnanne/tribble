fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);

    x = 6;
    println!("변경된 x의 값: {}", x);

    let y = 7;
    println!("y의 값: {}", y);

    // y = 8;
    // cannot assign twice to immutable variable - rustc(E0384)
}
