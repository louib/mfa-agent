#[macro_use]
extern crate gtk_macros;

use std::cell::RefCell;
use std::env;
use std::error::Error;

use clap::{AppSettings, Parser, Subcommand};
use gio::prelude::*;
use glib::{Receiver, Sender};
use gtk::prelude::*;
use libadwaita::prelude::*;
use libadwaita::subclass::prelude::*;
// use gtk_macros::send;

#[derive(Parser)]
#[clap(name = crate::consts::APP_NAME)]
// #[clap(author = "louib")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Multi-Factor authentication agent for Linux.", long_about = None)]
struct MFAAgent {
    /// Run the agent as a proxy
    #[clap(long, short)]
    proxy: bool,
}

mod api;
mod app;
mod bluetooth;
mod config;
mod connection;
mod consts;
mod logger;
mod tcp;
mod utils;
// mod numpad;
mod agent_window;
mod event;
mod proxy_window;
mod secrets;
mod secrets_window;
mod unlock_window;

fn get_connection_type() -> crate::connection::ConnectionType {
    match env::var(crate::consts::CONNECTION_TYPE_VAR_NAME) {
        Ok(v) => crate::connection::ConnectionType::from_string(&v).unwrap(),
        Err(_) => crate::connection::ConnectionType::Tcp,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();

    // let args = MFAAgent::parse();

    if let Err(e) = gio::resources_register_include!("ui.gresource") {
        panic!("Failed to register resources: {}.", e);
    }

    if let Err(e) = gtk::init() {
        panic!("Failed to initialize GTK: {}", e);
    }

    libadwaita::init();

    let connection_type = get_connection_type();
    log::info!("Connecting over {}", connection_type.to_string());

    // std::process::exit(run::<crate::numpad::NumPad>());

    if crate::app::is_proxy() {
        log::info!("Running in proxy mode!");

        match connection_type {
            crate::connection::ConnectionType::Bluetooth => {
                // TODO this should call ping instead
                tokio::spawn(crate::bluetooth::send_request_to_server(
                    "allo mon ami!!!".as_bytes().to_vec(),
                ));
            }
            crate::connection::ConnectionType::Tcp => {
                // TODO this should call ping instead
                // tokio::spawn(crate::tcp::send_data("allo mon ami!!!".as_bytes().to_vec()));
            }
            crate::connection::ConnectionType::Usb => {
                // TODO not implemented yet.
            }
        };

        // Do not open a database when a proxy.
    } else {
        let mut password: String = "".to_string();
        if true {
            // FIXME we should disable terminal echo here!!!
            // password = crate::utils::read_line("Please enter your password:");
            // We prompt from the command line for the password.
            // This option is only available when started from the command line!
        }

        // Else, we build the unlock UI and unlock with a UI!.

        log::info!("Running in remote agent mode!");
        match connection_type {
            crate::connection::ConnectionType::Bluetooth => {
                tokio::spawn(async {
                    if let Err(e) = crate::bluetooth::start_server().await {
                        log::error!("Error while running bluetooth server: {}", e);
                    } else {
                        log::info!("Bluetooth server has finished serving.");
                    }
                });
            }
            crate::connection::ConnectionType::Tcp => {
                tokio::spawn(async {
                    if let Err(e) = crate::tcp::start_server().await {
                        log::error!("Error while running TCP server: {}", e);
                    } else {
                        log::info!("TCP server has finished serving.");
                    }
                });
            }
            crate::connection::ConnectionType::Usb => {
                // TODO not implemented yet.
            }
        };
    }

    let (sender, receiver) =
        glib::MainContext::channel::<crate::event::ApplicationEvent>(glib::PRIORITY_DEFAULT);
    let receiver = RefCell::new(Some(receiver));

    receiver
        .borrow_mut()
        .take()
        .unwrap()
        .attach(None, handle_app_event);

    log::info!("Building GTK application {}", crate::app::get_app_id());
    crate::app::MFAAgentApplication::run();

    Ok(())
}

fn handle_app_event(event: crate::event::ApplicationEvent) -> glib::Continue {
    println!("Received application event {:?}", event);
    return glib::Continue(true);
}
