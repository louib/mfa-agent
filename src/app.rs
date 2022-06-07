use std::env;

use glib::subclass::InitializingObject;
use glib::{Object, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Button, CompositeTemplate, TemplateChild};
use libadwaita::subclass::prelude::*;

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
}

mod imp {
    use super::*;

    pub struct MFAAgentApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for MFAAgentApplication {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "MFAAgentApplication";
        type Type = super::MFAAgentApplication;
        type ParentType = libadwaita::Application;

        fn new() -> Self {
            Self {}
        }
    }

    impl ObjectImpl for MFAAgentApplication {}
    impl GtkApplicationImpl for MFAAgentApplication {}
    impl AdwApplicationImpl for MFAAgentApplication {}

    impl ApplicationImpl for MFAAgentApplication {
        fn activate(&self, app: &Self::Type) {
            let app = app.downcast_ref::<super::MFAAgentApplication>().unwrap();
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
