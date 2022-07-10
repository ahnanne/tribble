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
        println!("안녕히 가세요!");
        break;
    }
}
