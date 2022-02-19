use gtk::gio;

fn main() {
    gio::compile_resources(
        "ui/resources",
        "ui/resources/resources.gresource.xml",
        "ui.gresource",
    );
}
