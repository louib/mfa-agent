use std::error::Error;

// use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
// use btleplug::platform::{Adapter, Manager, Peripheral};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ListBox, Switch};
use tokio::time;

mod config;
mod logger;

const APP_ID: &str = "net.louib.mfa-agent";
const APP_NAME: &str = "mfa-agent";
const APP_TITLE: &str = "MFA Agent";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();

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
    let list_box = ListBox::new();

    let button = Button::builder()
        .label("Allow request for authentication?")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let switch = Switch::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    switch.connect_active_notify(move |switch| {
        println!("The value for the switch is now {}", switch.is_active());
    });

    list_box.append(&button);
    list_box.append(&switch);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&list_box)
        .build();

    window.present();
}
