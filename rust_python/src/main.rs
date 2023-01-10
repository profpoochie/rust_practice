use gtk::prelude::*;
use std::process::Command;
//use std::io::Write;
use std::process::Stdio;
use gtk::{Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Run python")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_button| {
        // command "ls" with arg "-l" on terminal
        let radius = 3.14;

        let  child = Command::new("python")
            .arg("path/to/script.py")
            .arg(radius.to_string())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to run Python script");

        let output = child.wait_with_output().expect("Failed to read stdout");
        let area = String::from_utf8(output.stdout).unwrap();
        println!("The area of the circle is: {}", area);
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}