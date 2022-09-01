use std::{
    cell::RefCell,
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
    rc::Rc,
};

use gtk::{prelude::*, ApplicationWindow, Menu, MenuItem, TextView};

pub(super) fn build_file_menu(textbox: &TextView, window: &ApplicationWindow) -> MenuItem {
    let filename = Rc::new(RefCell::new(PathBuf::new()));

    let file_item = MenuItem::with_label("File");
    let file_menu = Menu::new();

    let save_menu_item = build_save_menu_item(textbox, &filename);

    file_menu.append(&build_new_menu_item(textbox, &filename, &save_menu_item));
    file_menu.append(&build_open_menu_item(
        textbox,
        &filename,
        window,
        &save_menu_item,
    ));
    file_menu.append(&save_menu_item);
    file_menu.append(&build_save_as_menu_item(
        textbox,
        &filename,
        window,
        &save_menu_item,
    ));
    file_menu.append(&build_print_menu_item(textbox, window));
    file_menu.append(&build_exit_menu_item(window));

    file_item.set_submenu(Some(&file_menu));
    file_item
}

fn build_new_menu_item(
    textbox: &TextView,
    filename: &Rc<RefCell<PathBuf>>,
    save_menu_item: &MenuItem,
) -> MenuItem {
    let new_menu_item = MenuItem::with_label("New");
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
    new_menu_item
}

fn build_open_menu_item(
    textbox: &TextView,
    filename: &Rc<RefCell<PathBuf>>,
    window: &ApplicationWindow,
    save_menu_item: &MenuItem,
) -> MenuItem {
    let open_menu_item = MenuItem::with_label("Open");

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
                        reader.read_to_string(&mut contents).unwrap();

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
    open_menu_item
}

fn build_save_menu_item(textbox: &TextView, filename: &Rc<RefCell<PathBuf>>) -> MenuItem {
    let save_menu_item = MenuItem::with_label("Save");
    save_menu_item.set_sensitive(false);
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
    save_menu_item
}

fn build_save_as_menu_item(
    textbox: &TextView,
    filename: &Rc<RefCell<PathBuf>>,
    window: &ApplicationWindow,
    save_menu_item: &MenuItem,
) -> MenuItem {
    let save_as_menu_item = MenuItem::with_label("Save As");
    save_as_menu_item.connect_activate({
        let textbox = textbox.downgrade();
        let window = window.clone();
        let save_menu_item = save_menu_item.clone();
        let filename = filename.clone();
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
    save_as_menu_item
}

fn build_print_menu_item(textbox: &TextView, window: &ApplicationWindow) -> MenuItem {
    let print_menu_item = MenuItem::with_label("Print");
    print_menu_item.connect_activate({
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
    print_menu_item
}

fn build_exit_menu_item(window: &ApplicationWindow) -> MenuItem {
    let exit_menu_item = MenuItem::with_label("Exit");
    exit_menu_item.connect_activate({
        let window = window.clone();
        move |_| {
            window.close();
        }
    });
    exit_menu_item
}
