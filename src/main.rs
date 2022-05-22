use std::error::Error;
use std::time::Duration;

use bluer::adv::Advertisement;
use bluer::{Adapter, AdapterEvent, Address, DeviceEvent};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use gio::prelude::*;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ListBox, Switch};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::sleep,
};
use vgtk::run;

mod config;
mod consts;
mod logger;
mod numpad;
mod secrets;
mod secrets_window;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();

    if let Err(e) = gio::resources_register_include!("ui.gresource") {
        panic!("Failed to register resources: {}.", e);
    }

    if let Err(e) = gtk::init() {
        panic!("Failed to initialize GTK: {}", e);
    }

    std::process::exit(run::<crate::numpad::NumPad>());

    // advertise().await?;

    // Create a new application
    // let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    // app.connect_activate(build_ui);

    // Run the application
    // app.run();

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

pub struct BluetoothDevice {
    pub address: String,
    pub name: Option<String>,
}

pub enum ApplicationEvent {
    AddKnownDevice(String),
    PairWithDevice(String),
    UnpairWithDevice(String),
    AllowSecretAccess(String),
}

async fn handle_app_event(event: &ApplicationEvent) -> Result<(), String> {
    Ok(())
}

async fn discover_devices() -> bluer::Result<()> {
    log::info!("Discovering devices");

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    println!(
        "Discovering devices using Bluetooth adapater {}\n",
        adapter.name()
    );
    adapter.set_powered(true).await?;

    let device_events = adapter.discover_devices().await?;
    pin_mut!(device_events);

    loop {
        tokio::select! {
            Some(device_event) = device_events.next() => {
                match device_event {
                    AdapterEvent::DeviceAdded(addr) => {
                        let device = adapter.device(addr)?;
                        let name = device.name().await?;
                        println!("Device added: {} ({})", addr, name.unwrap_or("unknown".to_string()));
                    }
                    _ => (),
                }
            }
            else => break
        }
    }

    Ok(())
}

async fn advertise() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        adapter.name(),
        adapter.address().await?
    );
    let le_advertisement = Advertisement {
        advertisement_type: bluer::adv::Type::Peripheral,
        // FIXME change the UUID to something that is unique for this device.
        service_uuids: vec!["123e4567-e89b-12d3-a456-426614174000".parse().unwrap()]
            .into_iter()
            .collect(),
        discoverable: Some(true),
        local_name: Some("mfa-agent (remote)".to_string()),
        ..Default::default()
    };
    println!("{:?}", &le_advertisement);
    let handle = adapter.advertise(le_advertisement).await?;

    println!("Press enter to quit");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    println!("Removing advertisement");
    drop(handle);
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
