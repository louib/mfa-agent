// #![recursion_limit = "256"]
use std::cell::RefCell;
use std::env;
use std::error::Error;
use std::time::Duration;

use clap::{AppSettings, Parser, Subcommand};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use gio::prelude::*;
use glib::{Receiver, Sender};
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, Entry, Label, ListBox,
    StyleContext, Switch,
};
use gtk_macros::send;
use libadwaita::gdk::Display;
use libadwaita::prelude::*;
use libadwaita::subclass::prelude::*;
use log::error;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::sleep,
};
// use vgtk::run;

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
mod bluetooth;
mod config;
mod connection;
mod consts;
mod logger;
mod tcp;
mod utils;
// mod numpad;
mod secrets;
mod secrets_window;

fn is_proxy() -> bool {
    match env::var(crate::consts::IS_PROXY_VAR_NAME) {
        Ok(v) => v == "true",
        Err(_) => false,
    }
}

fn get_window_title() -> String {
    let mut app_title = crate::consts::APP_NAME.to_owned() + " ";
    if is_proxy() {
        app_title += crate::consts::PROXY_TITLE_SUFFIX;
    } else {
        app_title += crate::consts::AGENT_TITLE_SUFFIX;
    }
    app_title
}

fn get_connection_type() -> crate::connection::ConnectionType {
    match env::var(crate::consts::CONNECTION_TYPE_VAR_NAME) {
        Ok(v) => crate::connection::ConnectionType::from_string(&v).unwrap(),
        Err(_) => crate::connection::ConnectionType::Tcp,
    }
}

fn get_app_id() -> &'static str {
    match env::var(crate::consts::IS_DEV_VAR_NAME) {
        Ok(v) => {
            if v == "true" {
                crate::consts::DEV_APP_ID
            } else {
                crate::consts::APP_ID
            }
        }
        Err(_) => crate::consts::APP_ID,
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

    // crate::bluetooth::advertise().await?;
    //
    //
    if is_proxy() {
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
                        log::error!("Error while starting bluetooth server: {}", e);
                    } else {
                        log::info!("Bluetooth server has finished serving.");
                    }
                });
            }
            crate::connection::ConnectionType::Tcp => {
                tokio::spawn(async {
                    if let Err(e) = crate::tcp::start_server().await {
                        log::error!("Error while starting TCP server: {}", e);
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

    log::info!("Building GTK application {}", get_app_id());
    // Create a new application
    let app = Application::builder().application_id(get_app_id()).build();

    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak app => move |_action, _parameter| {
        app.quit();
    }));
    app.connect_startup(|app| {
        load_css();
        app.set_accels_for_action("app.quit", &["<Primary>Q"]);
    });

    app.add_action(&quit);

    // Connect to "activate" signal of `app`
    app.connect_activate(build_unlock_ui);
    // app.connect_activate(build_main_ui);
    // app.connect_activate(build_alt_ui);

    // Run the application
    app.run();

    Ok(())
}

fn build_unlock_ui(app: &Application) {
    let (sender, receiver) = glib::MainContext::channel::<ApplicationEvent>(glib::PRIORITY_DEFAULT);
    let receiver = RefCell::new(Some(receiver));

    receiver.borrow_mut().take().unwrap().attach(None, |event| {
        println!("Received application event {:?}", event);
        return glib::Continue(true);
    });

    let config = crate::config::read_or_init().expect("Could not load config.");

    let builder = gtk::Builder::from_string(include_str!("ui/unlock.ui"));

    // Get window and button from `gtk::Builder`
    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object `window` from builder.");
    window.set_title(Some(&get_window_title()));

    let select_label: Label = builder
        .object("select_label")
        .expect("Could not get the select label object from builder.");

    select_label.set_text("Please select a database to unlock");

    let select_button: Button = builder
        .object("select_button")
        .expect("Could not get the select button from builder.");
    select_button.set_halign(Align::End);
    select_button.set_valign(Align::End);

    if let Some(db_path) = config.default_db_path {
        select_label.set_text(&format!("Opening database at {}", &db_path));
    } else if let Some(db_path) = config.last_db_path {
        select_label.set_text(&format!("Opening database at {}", &db_path));
    } else {
        select_label.set_text("Please select a database to open.");
    }

    let submit_button: Button = builder
        .object("submit_button")
        .expect("Could not get the submit button from builder.");
    let password_field: Entry = builder
        .object("password")
        .expect("Could not get the password field from builder.");

    submit_button.set_halign(Align::End);
    submit_button.set_valign(Align::End);

    // Set application
    window.set_application(Some(app));

    password_field.connect_activate(move |password_field| {
        let password = password_field.text();
    });

    // Connect to "clicked" signal
    submit_button.connect_clicked(move |button| {
        let password = password_field.text();
        println!("Wow, the password is {}.", &password);
        send!(sender, ApplicationEvent::PasswordEntered(password.to_string()));
    });

    window.present();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("ui/style.css"));
    // If the current style is dark...
    // provider.load_from_data(include_bytes!("ui/style-dark.css"));
    // If the current style is high contrast...
    // provider.load_from_data(include_bytes!("ui/style-hc.css"));
    // If the current style is high contrast and dark...
    // provider.load_from_data(include_bytes!("ui/style-hc-dark.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_main_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("ui/main.ui"));

    // Get window and button from `gtk::Builder`
    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object `window` from builder.");
    window.set_title(Some(&get_window_title()));

    let list: ListBox = builder
        .object("list_box")
        .expect("Could not get object `window` from builder.");
    let button: Button = builder
        .object("button")
        .expect("Could not get object `button` from builder.");
    let button2: Button = builder
        .object("button_2")
        .expect("Could not get object `button` from builder.");
    let search_entry: Entry = builder
        .object("search_entry")
        .expect("Could not get object `search_entry` from builder.");
    let label: Label = builder
        .object("label")
        .expect("Could not get the label object from builder.");
    label.set_text("Would you accept request for XXXXXXXXXXX");

    // Set application
    window.set_application(Some(app));

    // Connect to "clicked" signal
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    search_entry.connect_activate(move |entry| {
        let text = entry.text();
        log::debug!("Got a query for text {}.", text);
        tokio::spawn(crate::tcp::search(text.to_string()));
        // TODO send the request to the server!!!
    });

    // Connect to "clicked" signal
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Add buttons
    window.set_child(Some(&list));
    window.present();
}

fn build_alt_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("ui/window.ui"));

    // Get window and button from `gtk::Builder`
    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object `window` from builder.");
    window.set_title(Some(&get_window_title()));

    // Set application
    window.set_application(Some(app));

    // Add buttons
    //window.set_child(Some(&list));
    window.present();
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum ApplicationEvent {
    PasswordEntered(String),
    AddKnownDevice(String),
    PairWithDevice(String),
    UnpairWithDevice(String),
    AllowSecretAccess(String),
}

async fn handle_app_event(event: &ApplicationEvent) -> Result<(), String> {
    Ok(())
}
