use glib::subclass::types::{InitializingObject, ObjectSubclass};
use glib::Object;
use gtk::subclass::widget::TemplateChild;
use gtk::{gio, glib};
use gtk::{Application, ApplicationWindow, Button, CompositeTemplate, Entry, ListBox, ListView, Switch};
//
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "../ui/search.ui")]
pub struct SecretsWindow {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub secrets_list_view: TemplateChild<ListView>,
    // pub model: OnceCell<gio::ListStore>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SecretsWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "SecretsWindow";
    type Type = super::SecretsWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
