use std::error::Error;

use bluer::{Adapter, AdapterEvent, Address, DeviceEvent};
// use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
// use btleplug::platform::{Manager, Peripheral};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use gio::prelude::*;
use gtk::prelude::WidgetExt;
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

pub enum ApplicationEvent {
    AddKnownDevice(String),
    PairWithDevice(String),
    UnpairWithDevice(String),
    AllowSecretAccess(String),
}

async fn handle_app_event(event: &ApplicationEvent) -> Result<(), String> {
    Ok(())
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

async fn discover_devices() -> bluer::Result<()> {
    let with_changes = true;
    let all_properties = true;

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    println!(
        "Discovering devices using Bluetooth adapater {}\n",
        adapter.name()
    );
    adapter.set_powered(true).await?;

    let device_events = adapter.discover_devices().await?;
    pin_mut!(device_events);

    let mut all_change_events = SelectAll::new();

    loop {
        tokio::select! {
            Some(device_event) = device_events.next() => {
                match device_event {
                    AdapterEvent::DeviceAdded(addr) => {
                        // if !filter_addr.is_empty() && !filter_addr.contains(&addr) {
                            // continue;
                        // }

                        println!("Device added: {}", addr);
                        let res = if all_properties {
                            query_all_device_properties(&adapter, addr).await
                        } else {
                            query_device(&adapter, addr).await
                        };
                        if let Err(err) = res {
                            println!("    Error: {}", &err);
                        }

                        if with_changes {
                            let device = adapter.device(addr)?;
                            let change_events = device.events().await?.map(move |evt| (addr, evt));
                            all_change_events.push(change_events);
                        }
                    }
                    AdapterEvent::DeviceRemoved(addr) => {
                        println!("Device removed: {}", addr);
                    }
                    _ => (),
                }
                println!();
            }
            Some((addr, DeviceEvent::PropertyChanged(property))) = all_change_events.next() => {
                println!("Device changed: {}", addr);
                println!("    {:?}", property);
            }
            else => break
        }
    }

    Ok(())
}
