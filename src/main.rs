//#![deny(clippy::too_many_lines)]
#![allow(clippy::redundant_clone)]

use gtk::gdk::Screen;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::prelude::*;

use gtk::{prelude::*, TextView};
use gtk::{AboutDialog, CssProvider, StyleContext};
use gtk::{Application, ApplicationWindow, Menu, MenuBar, MenuItem, ScrolledWindow};
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.Redit";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(move |_| load_css(15));
    app.connect_activate(build_ui);

    app.run();
}

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
    let menu_bar = MenuBar::new();

    menu_bar.append(&build_file_menu(&textbox, &window));
    menu_bar.append(&build_edit_menu());
    menu_bar.append(&build_view_menu());
    menu_bar.append(&build_about_menu());

    win_box.pack_start(&menu_bar, false, false, 0);
    scrolled_window.add(&textbox);
    win_box.pack_start(&scrolled_window, true, true, 0);
    window.add(&win_box);
    window.show_all();
}

fn build_edit_menu() -> MenuItem {
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

fn build_file_menu(textbox: &TextView, window: &ApplicationWindow) -> MenuItem {
    let filename = Rc::new(RefCell::new(PathBuf::new()));

    let file_item = MenuItem::with_label("File");
    let file_menu = Menu::new();
    let new_menu_item = MenuItem::with_label("New");
    file_menu.append(&new_menu_item);
    let save_menu_item = MenuItem::with_label("Save");
    save_menu_item.set_sensitive(false);
    new_menu_item.connect_activate({
        let textbox = textbox.clone();
        let filename = filename.clone();
        let save_menu_item = save_menu_item.clone();
        move |_| {
            textbox.buffer().expect("Couldn't get window").set_text("");
            *filename.borrow_mut() = PathBuf::new();
            save_menu_item.set_sensitive(false);
        }
    });
    let open_menu_item = MenuItem::with_label("Open");
    file_menu.append(&open_menu_item);
    open_menu_item.connect_activate({
        let textbox = textbox.clone();
        let filename = filename.clone();
        let save_menu_item = save_menu_item.clone();
        let window = window.clone();
        move |_| {
            let file_chooser = gtk::FileChooserDialog::new(
                Some("Open File"),
                Some(&window),
                gtk::FileChooserAction::Open,
            );

            file_chooser.add_buttons(&[
                ("Open", gtk::ResponseType::Ok),
                ("Cancel", gtk::ResponseType::Cancel),
            ]);

            file_chooser.connect_response({
                let textbox = textbox.clone();
                let filename = filename.clone();
                let save_menu_item = save_menu_item.clone();
                move |file_chooser, response| {
                    if response == gtk::ResponseType::Ok {
                        *filename.borrow_mut() =
                            file_chooser.filename().expect("Couldn't get filename");
                        let file = File::open(&*filename.borrow_mut()).expect("Couldn't open file");

                        let mut reader = BufReader::new(file);
                        let mut contents = String::new();
                        let _ = reader.read_to_string(&mut contents);

                        textbox
                            .buffer()
                            .expect("Couldn't get window")
                            .set_text(&contents);

                        save_menu_item.set_sensitive(true);
                    }
                    file_chooser.close();
                }
            });
            file_chooser.show_all();
        }
    });
    file_menu.append(&save_menu_item);
    save_menu_item.connect_activate({
        let textbox = textbox.clone();
        let filename = filename.clone();

        move |_| {
            let mut file = File::create(&*filename.borrow_mut()).expect("Couldn't open file");
            let textbuffer = textbox.buffer().unwrap();
            let (start_iter, end_iter) = textbuffer.bounds();
            write!(
                file,
                "{}",
                textbuffer.text(&start_iter, &end_iter, false).unwrap()
            )
            .unwrap();
        }
    });
    let save_as = MenuItem::with_label("Save As");
    file_menu.append(&save_as);
    save_as.connect_activate({
        let textbox = textbox.downgrade();
        let window = window.clone();
        let save_menu_item = save_menu_item.clone();
        move |_| {
            let textbox = match textbox.upgrade() {
                Some(textbox) => textbox,
                None => return,
            };
            let file_chooser = gtk::FileChooserDialog::new(
                Some("Save File"),
                Some(&window),
                gtk::FileChooserAction::Save,
            );

            file_chooser.add_buttons(&[
                ("Save", gtk::ResponseType::Ok),
                ("Cancel", gtk::ResponseType::Cancel),
            ]);

            file_chooser.connect_response({
                let textbox = textbox.clone();
                let filename = filename.clone();
                let save_menu_item = save_menu_item.clone();
                move |file_chooser, response| {
                    if response == gtk::ResponseType::Ok {
                        *filename.borrow_mut() =
                            file_chooser.filename().expect("Couldn't get filename");
                        let mut file =
                            File::create(&*filename.borrow_mut()).expect("Couldn't open file");
                        let textbuffer = textbox.buffer().unwrap();
                        let (start_iter, end_iter) = textbuffer.bounds();
                        write!(
                            file,
                            "{}",
                            textbuffer.text(&start_iter, &end_iter, false).unwrap()
                        )
                        .unwrap();

                        save_menu_item.set_sensitive(true);
                    }
                    file_chooser.close();
                }
            });
            file_chooser.show_all();
        }
    });
    let print = MenuItem::with_label("Print");
    file_menu.append(&print);
    print.connect_activate({
        let window = window.clone();
        let textbox = textbox.clone();
        move |_| {
            let print_operation = gtk::PrintOperation::new();
            print_operation.connect_begin_print({
                let textbox = textbox.clone();
                move |print_operation, _| {
                    let textbuffer = textbox.buffer().unwrap();
                    let end = textbuffer.end_iter();
                    let n_lines = end.line() + 1;

                    let num_pages = n_lines / 40;

                    print_operation.set_n_pages(num_pages + 1);
                }
            });

            print_operation.connect_draw_page({
                let textbox = textbox.clone();
                move |_, print_context, pg_num| {
                    let cairo = print_context
                        .cairo_context()
                        .expect("Couldn't get cairo context");

                    let font_description = pango::FontDescription::from_string("sans 14");
                    let pango_layout = print_context
                        .create_pango_layout()
                        .expect("Couldn't create pango layout");
                    pango_layout.set_font_description(Option::from(&font_description));

                    let textbuffer = textbox.buffer().unwrap();
                    let s_iter = textbuffer.iter_at_line(40 * pg_num);
                    let e_iter = textbuffer.iter_at_line(40 * (pg_num + 1));
                    pango_layout.set_text(&textbuffer.text(&s_iter, &e_iter, false).unwrap());
                    cairo.move_to(10.0, 10.0);
                    pangocairo::functions::show_layout(&cairo, &pango_layout);
                }
            });

            print_operation.set_allow_async(true);
            print_operation.connect_done(|_, _res| {
                //println!("printing done: {:?}", res);
            });

            print_operation
                .run(gtk::PrintOperationAction::PrintDialog, Some(&window))
                .expect("Couldn't print");
        }
    });
    let exit = MenuItem::with_label("Exit");
    file_menu.append(&exit);
    exit.connect_activate({
        let window = window.clone();
        move |_| {
            window.close();
        }
    });
    file_item.set_submenu(Some(&file_menu));
    file_item
}

fn build_about_menu() -> MenuItem {
    let about_item = MenuItem::with_label("About");
    let about_menu = Menu::new();

    let about_prog_item = MenuItem::with_label("About Program...");
    about_prog_item.connect_activate(move |_| {
        let logo = Pixbuf::from_file("src/rust.png").unwrap();

        let about_dialog = AboutDialog::builder()
            .logo(&logo)
            .program_name("Redit")
            .version("0.0.1")
            .comments("Notepad application written in Rust")
            .title("About Redit")
            .build();

        about_dialog.show_all();
    });

    about_menu.append(&about_prog_item);

    about_item.set_submenu(Some(&about_menu));
    about_item
}

fn build_view_menu() -> MenuItem {
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
