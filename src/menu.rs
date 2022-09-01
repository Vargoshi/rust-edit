use gtk::{prelude::*, ApplicationWindow, MenuBar, TextView};

mod about;
mod edit;
mod file;
mod view;

pub(super) fn build_menu(textbox: &TextView, window: &ApplicationWindow) -> MenuBar {
    let menu_bar = MenuBar::new();
    menu_bar.append(&file::build_file_menu(textbox, window));
    menu_bar.append(&edit::build_edit_menu());
    menu_bar.append(&view::build_view_menu());
    menu_bar.append(&about::build_about_menu());
    menu_bar
}
