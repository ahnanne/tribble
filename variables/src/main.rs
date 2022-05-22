fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);

    x = 6;
    println!("변경된 x의 값: {}", x);

    // x = "hi";
    // mismatched types - expected integer, found `&str` - rustc(E0308)

    let y = 7;
    println!("y의 값: {}", y);

    // y = 8;
    // cannot assign twice to immutable variable - rustc(E0384)

    // 변수 가리기
    let a = 1;
    let a = a * 2;
    let a = a * 10;
    println!("a의 값은 ... {}", a); // a의 값은 ... 20

    let a = "hi";
    println!("a의 값은 ... {}", a); // a의 값은 ... hi
}
