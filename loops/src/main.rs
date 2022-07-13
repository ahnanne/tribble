use rand::Rng;
mod sing_a_song;

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

    // * for -> 배열 같은 컬렉션 내의 요소들을 반복해서 처리해야 할 때 유용
    let aespa = ["Karina", "Giselle", "Winter", "NingNing"];

    for member in aespa.iter() {
        println!("나의 최애 멤버는 {}", member);
    }
    // while문을 사용해서 구현하는 것보다 안전성이 높고, 버그를 사전에 방지할 수 있다.

    // Range 타입 (지정한 범위의 값들을 생성해주는 타입)
    for count_down_number in (1..4).rev() {
        println!("{}!", count_down_number);
    }
    println!("{}", '🚀');

    // 🎶
    sing_a_song::twelve_days_of_christmas();
}
