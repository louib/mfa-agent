use std::cell::RefCell;
use std::env;

use gio::prelude::*;
use glib::subclass::InitializingObject;
use glib::{Object, Receiver, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Button, CompositeTemplate, CssProvider, StyleContext, TemplateChild};
use keepass::Database;
use libadwaita::gdk::Display;
use libadwaita::prelude::*;
use libadwaita::subclass::prelude::*;

use crate::widgets::agent_window::AgentWindow;
use crate::widgets::proxy_window::ProxyWindow;
use crate::widgets::unlock_window::UnlockWindow;

glib::wrapper! {
    pub struct MFAAgentApplication(ObjectSubclass<imp::MFAAgentApplication>)
        @extends gio::Application, gtk::Application, libadwaita::Application,
        @implements gio::ActionMap, gio::ActionGroup;

}

impl MFAAgentApplication {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create MFAAgentApplication")
    }

    pub fn run() {
        let app = glib::Object::new::<MFAAgentApplication>(&[
            ("application-id", &Some(get_app_id())),
            // The application flags are defined here https://docs.gtk.org/gio/flags.ApplicationFlags.html
            ("flags", &gio::ApplicationFlags::FLAGS_NONE),
            ("resource-base-path", &Some(crate::consts::RESOURCES_NAMESPACE)),
        ])
        .unwrap();

        app.run();
    }

    pub fn set_database(&mut self, db: Database) {
        let unlock_window = self.active_window().unwrap().close();

        println!("Opening database.");
        let window = AgentWindow::new(self);
        window.set_title(Some(&get_window_title()));
        window.set_database(db);
        window.present();
    }

    pub fn start_app() {
        if let Err(e) = gio::resources_register_include!("ui.gresource") {
            panic!("Failed to register resources: {}.", e);
        }

        if let Err(e) = gtk::init() {
            panic!("Failed to initialize GTK: {}", e);
        }

        libadwaita::init();

        let connection_type = get_connection_type();
        log::info!("Connecting over {}", connection_type.to_string());

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
        log::info!("GTK application has finished running");
    }
}

impl Default for MFAAgentApplication {
    fn default() -> Self {
        gio::Application::default()
            .expect("Could not get the running MFAAgentApplication instance")
            .downcast()
            .unwrap()
    }
}

mod imp {
    use super::*;

    #[derive(Default)]
    #[derive(Debug)]
    pub struct MFAAgentApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for MFAAgentApplication {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "MFAAgentApplication";
        type Type = super::MFAAgentApplication;
        type ParentType = libadwaita::Application;
    }

    impl ObjectImpl for MFAAgentApplication {}
    impl GtkApplicationImpl for MFAAgentApplication {}
    impl AdwApplicationImpl for MFAAgentApplication {}

    impl ApplicationImpl for MFAAgentApplication {
        fn activate(&self, app: &Self::Type) {
            let app = app.downcast_ref::<super::MFAAgentApplication>().unwrap();

            let quit = gio::SimpleAction::new("quit", None);
            quit.connect_activate(glib::clone!(@weak app => move |_action, _parameter| {
                app.quit();
            }));

            app.connect_startup(|app| {
                load_css();
                app.set_accels_for_action("app.quit", &["<Primary>Q"]);
            });
            app.add_action(&quit);

            if is_proxy() {
                let window = ProxyWindow::new(app);
                window.set_title(Some(&get_window_title()));
                window.present();
            } else {
                let mut unlock_window = UnlockWindow::new(app);
                unlock_window.set_title(Some(&get_window_title()));
                unlock_window.present();
            }
        }
    }
}

pub fn get_app_id() -> &'static str {
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

pub fn is_proxy() -> bool {
    match env::var(crate::consts::IS_PROXY_VAR_NAME) {
        Ok(v) => v == "true",
        Err(_) => false,
    }
}

pub fn get_window_title() -> String {
    let mut app_title = crate::consts::APP_NAME.to_owned() + " ";
    if is_proxy() {
        app_title += crate::consts::PROXY_TITLE_SUFFIX;
    } else {
        app_title += crate::consts::AGENT_TITLE_SUFFIX;
    }
    app_title
}

fn handle_app_event(event: crate::event::ApplicationEvent) -> glib::Continue {
    println!("Received application event {:?}", event);
    return glib::Continue(true);
}

fn get_connection_type() -> crate::connection::ConnectionType {
    match env::var(crate::consts::CONNECTION_TYPE_VAR_NAME) {
        Ok(v) => crate::connection::ConnectionType::from_string(&v).unwrap(),
        Err(_) => crate::connection::ConnectionType::Tcp,
    }
}
