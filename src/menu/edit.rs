use gtk::{prelude::*, Menu, MenuItem};

pub(super) fn build_edit_menu() -> MenuItem {
    let edit_item = MenuItem::with_label("Edit");
    let edit_menu = Menu::new();
    let undo_item = MenuItem::with_label("Undo");
    edit_menu.append(&undo_item);
    let cut_item = MenuItem::with_label("Cut");
    edit_menu.append(&cut_item);
    let copy_item = MenuItem::with_label("Copy");
    edit_menu.append(&copy_item);
    let paste_item = MenuItem::with_label("Paste");
    edit_menu.append(&paste_item);
    let delete_item = MenuItem::with_label("Delete");
    edit_menu.append(&delete_item);
    let find_item = MenuItem::with_label("Find");
    edit_menu.append(&find_item);
    let find_next_item = MenuItem::with_label("Find Next");
    edit_menu.append(&find_next_item);
    let replace_item = MenuItem::with_label("Replace");
    edit_menu.append(&replace_item);
    let go_to_item = MenuItem::with_label("Go to...");
    edit_menu.append(&go_to_item);
    let select_all_item = MenuItem::with_label("Select All");
    edit_menu.append(&select_all_item);
    edit_item.set_submenu(Some(&edit_menu));
    edit_item
}
