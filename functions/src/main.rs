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

    let x = first_five();
    let y = second_five();

    println!("{} + {} = {}", x, y, x + y);
}

fn first_five() -> i32 {
    // return 타입은 이와 같이 꼭 적어줘야 한다.
    return 5; // 이렇게 return 키워드와 함께 값을 반환해도 되지만,
}

fn second_five() -> i32 {
    5 // 대부분의 함수는 이렇게 마지막 표현식의 결과를 반환한다.
}

// 어떤 함수가 반환하려는 값이 없을 경우에는 빈 튜플인 ()로 표현된다.
