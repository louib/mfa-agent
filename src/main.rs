use std::error::Error;

// use bluer::{Adapter, AdapterEvent, Address, DeviceEvent};
use bluer::{Adapter, Address};
// use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
// use btleplug::platform::{Manager, Peripheral};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ListBox, Switch};
use tokio::time;

mod config;
mod logger;
mod secrets;
mod secrets_window;

const APP_ID: &str = "net.louib.mfa-agent";
const APP_NAME: &str = "mfa-agent";
const APP_TITLE: &str = "MFA Agent";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = gio::resources_register_include!("ui.gresource") {
        panic!("Failed to register resources.");
    }

    logger::init();

    if let Err(e) = gtk::init() {
        panic!("Failed to initialize GTK: {}", e);
    }

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
    // Init `gtk::Builder` from file
    let builder = gtk::Builder::from_string(include_str!("ui/main.ui"));

    // Get window and button from `gtk::Builder`
    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object `window` from builder.");
    let list: ListBox = builder
        .object("list_box")
        .expect("Could not get object `window` from builder.");
    let button: Button = builder
        .object("button")
        .expect("Could not get object `button` from builder.");
    let button2: Button = builder
        .object("button_2")
        .expect("Could not get object `button` from builder.");

    // Set application
    window.set_application(Some(app));

    // Connect to "clicked" signal
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Add buttons
    list.append(&button);
    list.append(&button2);
    window.set_child(Some(&list));
    window.present();
}

async fn query_device(adapter: &Adapter, addr: Address) -> bluer::Result<()> {
    let device = adapter.device(addr)?;
    println!("    Address type:       {}", device.address_type().await?);
    println!("    Name:               {:?}", device.name().await?);
    println!(
        "    UUIDs:              {:?}",
        device.uuids().await?.unwrap_or_default()
    );
    println!("    RSSI:               {:?}", device.rssi().await?);
    Ok(())
}

async fn query_all_device_properties(adapter: &Adapter, addr: Address) -> bluer::Result<()> {
    let device = adapter.device(addr)?;
    let props = device.all_properties().await?;
    for prop in props {
        println!("    {:?}", &prop);
    }
    Ok(())
}
