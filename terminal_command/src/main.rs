use gtk::prelude::*;
use std::process::Command;
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
        .label("Command ls")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_button| {
        // command "ls" with arg "-l" on terminal
        let output = Command::new("ls")
            .arg("-l")
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
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


