use glib::Object;
use glib::subclass::InitializingObject;
use gtk::{gio, glib, Application, Button, TemplateChild, CompositeTemplate};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct ProxyWindow(ObjectSubclass<imp::ProxyWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ProxyWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create ProxyWindow")
    }
}

mod imp {
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/net/louib/mfa-agent/proxy_window.ui")]
    pub struct ProxyWindow {
        #[template_child]
        pub button: TemplateChild<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProxyWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "ProxyWindow";
        type Type = super::ProxyWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProxyWindow {
        fn constructed(&self, obj: &Self::Type) {
            // Call "constructed" on parent
            self.parent_constructed(obj);

            // Connect to "clicked" signal of `button`
            self.button.connect_clicked(move |button| {
                // Set the label to "Hello World!" after the button has been clicked on
                button.set_label("Hello World!");
            });
        }
    }

    impl WidgetImpl for ProxyWindow {}
    impl ApplicationWindowImpl for ProxyWindow {}
    impl WindowImpl for ProxyWindow {}
}
