#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let name: &str = "Alex";
    let mut age: i32 = 32;
    let amount: i64 = 5115112120202;

    age = 43;
    println!("age {}", age);

    let color: &str = "blue";
    let color: i32 = 86;

    println!("{}", color);

    let (a, b, c) = (43, 85, "red");
}
