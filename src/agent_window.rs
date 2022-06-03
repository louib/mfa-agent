use glib::subclass::InitializingObject;
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Button, CompositeTemplate, TemplateChild};
use libadwaita::subclass::prelude::*;

glib::wrapper! {
    pub struct AgentWindow(ObjectSubclass<imp::AgentWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AgentWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create AgentWindow")
    }
}

mod imp {
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/net/louib/mfa-agent/agent_window.ui")]
    pub struct AgentWindow {
        // #[template_child]
        // pub submit_button: TemplateChild<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AgentWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "AgentWindow";
        type Type = super::AgentWindow;
        type ParentType = libadwaita::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AgentWindow {
        fn constructed(&self, obj: &Self::Type) {
            // Call "constructed" on parent
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for AgentWindow {}
    impl ApplicationWindowImpl for AgentWindow {}
    impl WindowImpl for AgentWindow {}
    impl AdwApplicationWindowImpl for AgentWindow {}

    #[gtk::template_callbacks]
    impl AgentWindow {
        #[template_callback]
        fn handle_button_clicked(button: &Button) {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
        }
    }
}
