use gtk::gio;

fn main() {
    println!("Compiling gresources.");
    gio::compile_resources("src/ui", "src/ui/resources.gresource.xml", "ui.gresource");
}
