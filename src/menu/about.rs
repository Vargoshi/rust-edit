use gtk::{gdk_pixbuf::Pixbuf, prelude::*, AboutDialog, Menu, MenuItem};

pub(super) fn build_about_menu() -> MenuItem {
    let about_item = MenuItem::with_label("About");
    let about_menu = Menu::new();

    let about_prog_item = MenuItem::with_label("About Program...");
    about_prog_item.connect_activate(move |_| {
        let logo = Pixbuf::from_file("src/rust.png").unwrap();

        let about_dialog = AboutDialog::builder()
            .logo(&logo)
            .program_name("Redit")
            .version("0.0.2")
            .comments("Notepad application written in Rust")
            .title("About Redit")
            .build();

        about_dialog.show_all();
    });

    about_menu.append(&about_prog_item);

    about_item.set_submenu(Some(&about_menu));
    about_item
}
