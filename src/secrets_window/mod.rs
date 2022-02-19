use gtk::subclass::widget::TemplateChild;
use gtk::{gio, glib};
use gtk::{Application, CompositeTemplate};

mod imp {
    use glib::subclass::object::ObjectImpl;
    use glib::subclass::prelude::ObjectSubclass;

    #[derive(Default)]
    // #[template(file = "../ui/search.ui")]
    pub struct SecretsWindow;

    #[glib::object_subclass]
    impl ObjectSubclass for SecretsWindow {
        const NAME: &'static str = "SecretsWindow";

        type Type = super::SecretsWindow;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for SecretsWindow {}
}

glib::wrapper! {
    pub struct SecretsWindow(ObjectSubclass<imp::SecretsWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl SecretsWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        glib::Object::new(&[("application", app)]).expect("Failed to create Window")
    }
}
