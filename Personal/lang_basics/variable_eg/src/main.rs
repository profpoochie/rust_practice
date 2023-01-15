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

    let pi: f32 = 3.1416;
    println!("{}", pi);

    let million = 1_000_000;
    println!("{}", million);

    let is_day = true;
    let is_night = false;
    println!("{}", is_day);

    let char1 = 'A';
    let smiley_face = '\u{1f601}';
    println!("characters {} and {}", char1, smiley_face);
}
