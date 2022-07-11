use rand::Rng;

// Rust가 제공하는 루프의 종류: loop, while, for

fn main() {
    let mut counter = 0;

    // * loop -> 반복을 중지하라고 명시적으로 설정하지 않는 한, 코드 블록을 무한 반복
    let result = loop {
        counter += 1;

        if counter > 12 {
            break counter * 2;
        }
    };

    println!("결과는!.. {}", result); // 26

    // * while -> 조건식의 평가 결과에 따라 루프를 실행해야 할 때 사용
    let mut number = 3;

    while number >= 0 {
        number -= 1;
        let random_number: i32 = rand::thread_rng().gen_range(1..100);
        println!("{}", random_number);
    }
}
