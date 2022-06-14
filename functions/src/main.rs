fn main() {
    println!("숫자 더하기");

    get_date(2022, 10, 5);
}

fn get_date(year: i32, month: i32, day: i32) {
    let a = 1000;

    let b = {
        let sum = year + month + day; // 구문 -> 값을 반환하지 않는다!
        sum - 2000 // 표현식 -> 세미콜론을 포함하지 않는다!
    };

    println!("a 더하기 b는...: {}", a + b);
}
