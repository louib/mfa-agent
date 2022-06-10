use std::env;

use glib::subclass::InitializingObject;
use glib::{Object, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Button, CompositeTemplate, CssProvider, StyleContext, TemplateChild};
use keepass::Database;
use libadwaita::gdk::Display;
use libadwaita::subclass::prelude::*;

use crate::agent_window::AgentWindow;
use crate::proxy_window::ProxyWindow;
use crate::unlock_window::UnlockWindow;

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
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &Some("/net/louib/mfa-agent/")),
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
                let mut unlock_window = UnlockWindow::new(app);
                unlock_window.set_title(Some(&get_window_title()));
                unlock_window.present();

                // let window = ProxyWindow::new(app);
                // window.set_title(Some(&get_window_title()));
                // window.present();
            } else {
                let window = AgentWindow::new(app);
                window.set_title(Some(&get_window_title()));
                window.present();
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
