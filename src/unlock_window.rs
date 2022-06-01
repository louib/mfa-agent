use glib::subclass::InitializingObject;
use glib::Object;
use gtk::prelude::*;
use libadwaita::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Button, CompositeTemplate, TemplateChild};

glib::wrapper! {
    pub struct UnlockWindow(ObjectSubclass<imp::UnlockWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl UnlockWindow {
    pub fn new(app: &Application) -> Self {
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
        fn handle_button_clicked(button: &Button) {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
        }
    }
}
