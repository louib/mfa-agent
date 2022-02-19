use gtk::subclass::widget::TemplateChild;
use gtk::{gio, glib};
use gtk::{CompositeTemplate};

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
    pub struct SecretsWindow(ObjectSubclass<imp::SecretsWindow>);
}

impl SecretsWindow {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}
