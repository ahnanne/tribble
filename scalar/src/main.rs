fn main() {
    /* 부동 소수점 타입 */
    let x = 2.0; // f64
    let y: f32 = 3.0;
    // 소수점 가진 숫자 처리할 때 따로 타입 명시하지 않으면 기본적으로 f64 타입으로 추론됨.
    println!("{} {}", x, y);

    /* 사칙 연산 */
    let sum = 1 + 1;
    println!("1 더하기 1은? = {}", sum); // 2

    let diff = 352.2 - 17.7;
    println!("{}", diff); // 334.5

    // let product = 2 * 2.3;
    // cannot multiply `{integer}` by `{float}` - rustc(E0277)
    let product = 2.0 * 2.3;
    println!("{}", product); // 4.6

    let remainder = 20 % 3;
    println!("{}", remainder); // 2
}
