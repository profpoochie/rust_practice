#[allow(unused_variables)]
#[allow(unused_assignments)]

const URL: &str = "google.com";

fn main() {
    /*
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
    println!("characters {} and {}", char1, smiley_face); */

    // String slices immutable
    let cat: &'static str = "Fluffy";
    println!("{}", cat);

    // String objects mutable
    let dog = String::new();
    let mut dog = String::from("Max");
    println!("{}", dog);

    // format! macro
    let owner = format!("Hi I'm {} the owner of {}", "Mark", dog);
    println!("{}", owner);

    // length of the string
    println!("length of the string {}", dog.len());

    // push & push string
    dog.push(' ');
    dog.push_str("the dog");
    println!("{}", dog);

    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog);

    println!("{}", URL);
}
