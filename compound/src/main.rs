/** 변수의 타입을 출력하는 함수 */
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable

fn main() {
    let tup1 = (910, 33.3, 9);
    // 다음과 같이 type annotation을 적용하는 것도 가능
    let tup2: (i32, f64, u8) = (910, 33.3, 9);

    print_type_of(&tup1); // (i32, f64, i32)
    print_type_of(&tup2); // (i32, f64, u8)

    let (_, a, b) = tup2;
    println!("a + b = {}", a + f64::from(b)); // a + b = 42.3

    let nine1 = tup1.2;
    let nine2 = tup2.2;
    // ㄱ.
    print_type_of(&nine1); // i32
    print_type_of(&nine2); // u8

    // ㄴ.
    println!("nine1 == nine2 일까요? 정답은 {} 입니다.", nine1 == nine2);
    // nine1 == nine2 일까요? 정답은 true 입니다.

    // ㄴ.을 주석 처리하면 ㄱ.의 출력 결과는 i32, u8이며,
    // ㄴ.을 주석 해제하면 ㄱ.의 출력 결과는 u8, u8이다.
}
