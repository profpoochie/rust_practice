use std::collections::HashMap;

fn main() {
    let mut person = HashMap::new();

    person.insert("first_name".to_string(), "John".to_string());
    person.insert("last_name".to_string(), "Doe".to_string());
    person.insert("age".to_string(), 32.to_string());

    let mut address = HashMap::new();
    address.insert("street".to_string(), "123 Main St.".to_string());
    address.insert("city".to_string(), "Anytown".to_string());
    address.insert("state".to_string(), "CA".to_string());
    address.insert("zip".to_string(), "12345".to_string());

    let mut superhash = HashMap::new();

    superhash.insert("address".to_string(), address);

    println!("{}", person.get("first_name").unwrap()); // Output: John
    println!("{}", superhash.get("address").unwrap().get("city").unwrap()); // Output: Anytown
}