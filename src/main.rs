use gtk::*;
use std::process;

pub struct Header {
    pub container: HeaderBar,
}
impl Header {
    fn new() -> Header {
        // Creates the main header bar container widget.
        let container = HeaderBar::new();

        // Sets the text to display in the title section of the header bar.
        container.set_title(Some("App Name"));
        // Enable the window controls within this headerbar.
        container.set_show_close_button(true);

        // Returns the header and all of it's state
        Header { container }
    }
}
pub struct App {
    pub window: Window,
    pub header: Header,
}

impl App {
    fn new() -> App {
        // Create a new top level window.
        let window = Window::new(WindowType::Toplevel);
        // Create a the headerbar and it's associated content.
        let header = Header::new();

        // Set the headerbar as the title bar widget.
        window.set_titlebar(Some(&header.container));
        // Set the title of the window.
        window.set_title("App Name");
        // Set the window manager class.
        window.set_wmclass("app-name", "App name");
        // The icon the app will display.
        Window::set_default_icon_name("iconname");

        // Programs what to do when the exit button is used.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Return our main application state
        App { window, header }
    }
}

fn main() {
    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    // Initialize the UI's initial state
    let app = App::new();

    // Make all the widgets within the UI visible.
    app.window.show_all();

    // Start the GTK main event loop
    gtk::main();
}
