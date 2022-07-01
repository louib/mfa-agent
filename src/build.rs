#[cfg(feature = "gtk")]
use gtk::gio;

fn main() {
    println!("Compiling gresources.");
    #[cfg(feature = "gtk")]
    gio::compile_resources("src/ui", "src/ui/resources.gresource.xml", "ui.gresource");
}
