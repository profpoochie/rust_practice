use std::process::Command;

fn term_command(input_string: String) {
    let input_vec: Vec<&str> = input_string.split(' ').collect();
    let command = input_vec[0];
    let args = &input_vec[1..];

    Command::new("gnome-terminal")
        .arg("-e")
        .arg(format!("sh -c '{} {:?}; read -p \"Press any key to continue...\"'", command, args).as_str())
        .spawn()
        .expect("Failed to open new terminal");
}

fn main() {
    let input_string = String::from("ls -lh");
    term_command(input_string);
}