use std::error::Error;

// use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
// use btleplug::platform::{Adapter, Manager, Peripheral};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use tokio::time;

const APP_ID: &str = "net.louib.mfa-agent";
const APP_NAME: &str = "mfa-agent";
const APP_TITLE: &str = "MFA Agent";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();

    Ok(())
}

pub enum Numbers {
    Zero,
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Allow request for authentication?")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&button)
        .build();

    // Present window
    window.present();
}
