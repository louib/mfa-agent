use gtk::subclass::widget::TemplateChild;
use gtk::{gio, glib};
use gtk::{Application, Button};

mod imp {
    use glib::subclass::object::ObjectImpl;
    use glib::subclass::prelude::ObjectSubclass;
    use gtk::subclass::prelude::{WidgetImpl, TemplateChild};
    use gtk::subclass::widget::WidgetClassSubclassExt;
    use gtk::{Button, CompositeTemplate};

    #[derive(CompositeTemplate, Default)]
    #[template(file = "./template.ui")]
    pub struct SecretsWindow {
        #[template_child]
        pub button: TemplateChild<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SecretsWindow {
        const NAME: &'static str = "SecretsWindow";

        type Type = super::SecretsWindow;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for SecretsWindow {}
    impl WidgetImpl for SecretsWindow {}
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
