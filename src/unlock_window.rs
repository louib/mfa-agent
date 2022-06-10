use std::env;
use std::fs::File;
use std::path::Path;

use glib::subclass::InitializingObject;
use glib::{Object, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Button, CompositeTemplate, Entry, TemplateChild};
use keepass::Database;
use libadwaita::subclass::prelude::*;

glib::wrapper! {
    pub struct UnlockWindow(ObjectSubclass<imp::UnlockWindow>)
        @extends libadwaita::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl UnlockWindow {
    pub fn new(app: &crate::app::MFAAgentApplication) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create UnlockWindow")
    }
}

mod imp {
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/net/louib/mfa-agent/unlock_window.ui")]
    pub struct UnlockWindow {
        #[template_child]
        pub submit_button: TemplateChild<Button>,

        #[template_child]
        pub password_entry: TemplateChild<Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UnlockWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "UnlockWindow";
        type Type = super::UnlockWindow;
        type ParentType = libadwaita::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UnlockWindow {
        fn constructed(&self, obj: &Self::Type) {
            // Call "constructed" on parent
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for UnlockWindow {}
    impl ApplicationWindowImpl for UnlockWindow {}
    impl WindowImpl for UnlockWindow {}
    impl AdwApplicationWindowImpl for UnlockWindow {}

    #[gtk::template_callbacks]
    impl UnlockWindow {
        #[template_callback]
        fn handle_password_submit(&self, button: &Button) {
            let password = self.password_entry.text();

            let open_database_future = async move {
                // let imp = this.imp();

                let db_path = get_db_path();

                let path = std::path::Path::new(&db_path);
                let db = match Database::open(&mut File::open(path).unwrap(), Some(&password), None) {
                    Ok(db) => db,
                    Err(e) => {
                        // TODO this should be a UI message instead.
                        log::warn!("Could not open database.");
                        return;
                    }
                };

                println!("There are {} entries in this database.", db.root.children.len());

                let app = crate::app::MFAAgentApplication::default();
                app.open_database();
            };
            spawn!(open_database_future);
        }
    }
}

fn get_db_path() -> String {
    match env::home_dir() {
        Some(d) => format!("{}/mfa-agent.kdbx", d.to_str().unwrap()),
        None => "./mfa-agent.kdbx".to_string(),
    }
}
