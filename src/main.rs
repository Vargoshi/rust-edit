#![deny(clippy::pedantic)]
#![allow(clippy::redundant_clone)]

use gtk::{gio::prelude::*, prelude::*, Application, ApplicationWindow, ScrolledWindow, TextView};

mod menu;

const APP_ID: &str = "org.gtk_rs.Redit";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Redit")
        .resizable(true)
        .build();

    let textbox = TextView::new();

    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_size_request(640, 400);

    let win_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let menu_bar = menu::build_menu(&textbox, &window);
    win_box.pack_start(&menu_bar, false, false, 0);
    scrolled_window.add(&textbox);
    win_box.pack_start(&scrolled_window, true, true, 0);
    window.add(&win_box);
    window.show_all();
}
