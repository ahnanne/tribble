fn main() {
    println!("오버워치2 출시일은!?");

    get_date(2022, 10, 5);
}

fn get_date(year: i32, month: i32, day: i32) {
    println!("{}년 {}월 {}일", year, month, day);
}
