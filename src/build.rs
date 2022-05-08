use gtk::gio;

fn main() {
    println!("Compiling gresources.");
    gio::compile_resources(
        "ui/resources",
        "ui/resources/resources.gresource.xml",
        "ui.gresource",
    );
}
