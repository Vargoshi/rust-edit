use std::{cell::RefCell, rc::Rc};

use gtk::{gdk::Screen, prelude::*, CssProvider, Menu, MenuItem, StyleContext};

fn load_css(size: i32) {
    // Load the CSS file and add it to the provider
    let css_code = format!("textview {{ font-size: {}px; }}", size);
    let provider = CssProvider::new();
    provider.load_from_data(css_code.as_bytes()).unwrap();

    StyleContext::remove_provider_for_screen(
        &Screen::default().expect("Could not connect to a display."),
        &provider,
    );
    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub(super) fn build_view_menu() -> MenuItem {
    load_css(15);

    let view_item = MenuItem::with_label("View");
    let view_menu = Menu::new();
    view_item.set_submenu(Some(&view_menu));
    let font_size = Rc::new(RefCell::new(15));
    let zoom_in_item = MenuItem::with_label("Zoom In");
    zoom_in_item.connect_activate({
        let font_size = font_size.clone();
        move |_| {
            *font_size.borrow_mut() += 2;
            load_css(*font_size.borrow_mut());
        }
    });
    view_menu.append(&zoom_in_item);
    let zoom_out_item = MenuItem::with_label("Zoom Out");
    zoom_out_item.connect_activate({
        let font_size = font_size.clone();
        move |_| {
            *font_size.borrow_mut() -= 2;
            load_css(*font_size.borrow_mut());
        }
    });
    view_menu.append(&zoom_out_item);
    let default_zoom_item = MenuItem::with_label("Restore Default Zoom");
    default_zoom_item.connect_activate({
        let font_size = font_size.clone();
        move |_| {
            *font_size.borrow_mut() = 15;
            load_css(*font_size.borrow_mut());
        }
    });
    view_menu.append(&default_zoom_item);
    view_item
}
