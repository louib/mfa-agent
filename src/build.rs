use gtk::gio;

fn main() {
    println!("Compiling gresources.");
    gio::compile_resources(
        "src/ui/resources",
        "src/ui/resources/resources.gresource.xml",
        "ui.gresource",
    );
}
