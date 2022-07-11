// Rust가 제공하는 루프의 종류: loop, while, for

// * loop -> 반복을 중지하라고 명시적으로 설정하지 않는 한, 코드 블록을 무한 반복
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 12 {
            break counter * 2;
        }
    };

    println!("결과는!.. {}", result); // 26
}
