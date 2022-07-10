use std::io;

fn main() {
    let mut input_number: String = String::new(); // creates a new empty String

    loop {
        println!("숫자를 입력해주세요!");
        io::stdin()
            .read_line(&mut input_number)
            .expect("입력한 값을 읽지 못했습니다.");

        let input_number = match input_number.trim().parse::<i32>() {
            Ok(number) => number,
            Err(_) => continue,
        };

        // if문의 조건은 반드시 bool 타입을 반환해야 한다.
        if input_number < 5 {
            println!("입력하신 값이 5보다 작네요!");
        } else {
            println!("입력하신 값이 5보다 크거나 같네요!");
        }

        // [if 표현식의 결과를 변수에 대입하기]
        // if 표현식은 표현식이기 때문에 let 구문의 오른쪽에 사용하는 것도 가능하다.
        // 이때 if 구문의 각 가지가 반환하는 값은 모두 같은 타입이어야 한다.
        let result = if input_number < 5 {
            '🐱'
        } else {
            '🐶'
        };

        println!("결과 : {}", result);

        break;
    }
}
