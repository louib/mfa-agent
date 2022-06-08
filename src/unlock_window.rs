use glib::subclass::InitializingObject;
use glib::{Object, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Button, CompositeTemplate, Entry, TemplateChild};
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
            // Set the label to "Hello World!" after the button has been clicked on
            let app = crate::app::MFAAgentApplication::default();

            let password = self.password_entry.text();
            println!("The current password is {}", password);
            app.open_database();
        }
    }
}
