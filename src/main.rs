use gtk::gdk::Screen;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::prelude::*;
use gtk::glib::clone;
use gtk::{glib, AboutDialog, CssProvider, StyleContext};
use gtk::{prelude::*, TextView};
use gtk::{Application, ApplicationWindow, Menu, MenuBar, MenuItem, ScrolledWindow};
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.Redit";

struct CSSProperties {
    font: i32,
}

struct FileProperties {
    filename: PathBuf,
}

struct PageProperties {
    number: i32,
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(move |_| load_css(15));
    app.connect_activate(build_ui);

    app.run();
}

fn load_css(size: i32) {
    // Load the CSS file and add it to the provider
    let part1 = "textview {
        font-size: "
        .to_string();
    let size_string = size.to_string();
    let part2 = "px;
}"
    .to_string();
    let css_code = part1 + &size_string + &part2;
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
    let css = Rc::new(RefCell::new(CSSProperties { font: 15 }));
    let filename = Rc::new(RefCell::new(FileProperties {
        filename: PathBuf::new(),
    }));

    let page_num = Rc::new(RefCell::new(PageProperties { number: 0 }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Redit")
        .resizable(true)
        .build();

    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_size_request(640, 400);

    let win_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let menu_bar = MenuBar::new();

    let file = MenuItem::with_label("File");
    let file_menu = Menu::new();

    let new = MenuItem::with_label("New");
    let open = MenuItem::with_label("Open");
    let save = MenuItem::with_label("Save");
    let save_as = MenuItem::with_label("Save As");
    let print = MenuItem::with_label("Print");
    let exit = MenuItem::with_label("Exit");

    let edit = MenuItem::with_label("Edit");
    let edit_menu = Menu::new();

    let undo = MenuItem::with_label("Undo");
    let cut = MenuItem::with_label("Cut");
    let copy = MenuItem::with_label("Copy");
    let paste = MenuItem::with_label("Paste");
    let delete = MenuItem::with_label("Delete");
    let find = MenuItem::with_label("Find");
    let find_next = MenuItem::with_label("Find Next");
    let replace = MenuItem::with_label("Replace");
    let go_to = MenuItem::with_label("Go to...");
    let select_all = MenuItem::with_label("Select All");

    let view = MenuItem::with_label("View");
    let view_menu = Menu::new();

    let zoom_in = MenuItem::with_label("Zoom In");
    let zoom_out = MenuItem::with_label("Zoom Out");
    let default_zoom = MenuItem::with_label("Restore Default Zoom");

    let about = MenuItem::with_label("About");
    let about_menu = Menu::new();

    let about_prog = MenuItem::with_label("About Program...");

    file_menu.append(&new);
    file_menu.append(&open);
    file_menu.append(&save);
    file_menu.append(&save_as);
    file_menu.append(&print);
    file_menu.append(&exit);
    file.set_submenu(Some(&file_menu));
    menu_bar.append(&file);

    edit_menu.append(&undo);
    edit_menu.append(&cut);
    edit_menu.append(&copy);
    edit_menu.append(&paste);
    edit_menu.append(&delete);
    edit_menu.append(&find);
    edit_menu.append(&find_next);
    edit_menu.append(&replace);
    edit_menu.append(&go_to);
    edit_menu.append(&select_all);
    edit.set_submenu(Some(&edit_menu));
    menu_bar.append(&edit);

    view_menu.append(&zoom_in);
    view_menu.append(&zoom_out);
    view_menu.append(&default_zoom);
    view.set_submenu(Some(&view_menu));
    menu_bar.append(&view);

    about_menu.append(&about_prog);
    about.set_submenu(Some(&about_menu));
    menu_bar.append(&about);

    win_box.pack_start(&menu_bar, false, false, 0);

    let textbox = TextView::new();

    scrolled_window.add(&textbox);

    win_box.pack_start(&scrolled_window, true, true, 0);

    window.add(&win_box);

    save.set_sensitive(false);

    window.show_all();

    let filename_clear = Rc::clone(&filename);

    new.connect_activate(clone!(@weak save, @weak textbox => move |_| {
        textbox.buffer().expect("Couldn't get window").set_text("");
        filename_clear.borrow_mut().filename = PathBuf::new();
        save.set_sensitive(false);
    }));

    let filename_copy = Rc::clone(&filename);

    open.connect_activate(clone!(@weak save, @weak textbox, @weak window => move |_| {
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            gtk::FileChooserAction::Open,
        );

        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);

        let filename1 = Rc::clone(&filename_copy);

        file_chooser.connect_response(clone!(@weak textbox => move |file_chooser, response| {
            if response == gtk::ResponseType::Ok {

                filename1.borrow_mut().filename = file_chooser.filename().expect("Couldn't get filename");
                let file = File::open(&filename1.borrow_mut().filename).expect("Couldn't open file");

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

                textbox
                    .buffer()
                    .expect("Couldn't get window")
                    .set_text(&contents);

                    save.set_sensitive(true);
            }
            file_chooser.close();
        }));
        file_chooser.show_all();
    }));

    let filename2 = Rc::clone(&filename);

    save.connect_activate(clone!(@weak textbox, @weak window => move |_| {
        let mut file = File::create(&filename2.borrow_mut().filename).expect("Couldn't open file");
        let textbuffer = textbox.buffer().unwrap();
        let (start_iter, end_iter) = textbuffer.bounds();
        write!(file, "{}", textbuffer.text(&start_iter, &end_iter, false).unwrap()).unwrap();
    }));

    let filename_copy2 = Rc::clone(&filename);

    save_as.connect_activate(clone!(@weak save, @weak textbox, @weak window => move |_| {
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Save File"),
            Some(&window),
            gtk::FileChooserAction::Save,
        );

        file_chooser.add_buttons(&[
            ("Save", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);

        let filename3 = Rc::clone(&filename_copy2);

        file_chooser.connect_response(clone!(@weak textbox => move |file_chooser, response| {
            if response == gtk::ResponseType::Ok {
                filename3.borrow_mut().filename = file_chooser.filename().expect("Couldn't get filename");
                let mut file = File::create(&filename3.borrow_mut().filename).expect("Couldn't open file");
                let textbuffer = textbox.buffer().unwrap();
                let (start_iter, end_iter) = textbuffer.bounds();
                write!(file, "{}", textbuffer.text(&start_iter, &end_iter, false).unwrap()).unwrap();

                save.set_sensitive(true);
            }
            file_chooser.close();
        }));
        file_chooser.show_all();
    }));

    print.connect_activate(clone!(@weak textbox, @weak window => move |_| {

        let print_operation = gtk::PrintOperation::new();
        print_operation.connect_begin_print(clone!(@weak textbox => move |print_operation, _| {

            let textbuffer = textbox.buffer().unwrap();
            let end = textbuffer.end_iter();
            let n_lines = end.line()+1;

            let num_pages = n_lines/40;


            print_operation.set_n_pages(num_pages+1);
        }));

        let page_num1 = Rc::clone(&page_num);
        page_num1.borrow_mut().number=0;

        print_operation.connect_draw_page(move |_, print_context, _| {
            let cairo = print_context
                .cairo_context()
                .expect("Couldn't get cairo context");

            let font_description = pango::FontDescription::from_string("sans 14");
            let pango_layout = print_context
                .create_pango_layout()
                .expect("Couldn't create pango layout");
            pango_layout.set_font_description(Option::from(&font_description));

            let textbuffer = textbox.buffer().unwrap();
            let s_iter = textbuffer.iter_at_line(40*page_num1.borrow_mut().number);
            let e_iter = textbuffer.iter_at_line(40*(page_num1.borrow_mut().number+1));
            pango_layout.set_text(&textbuffer.text(&s_iter, &e_iter, false).unwrap());
            cairo.move_to(10.0, 10.0);
            pangocairo::functions::show_layout(&cairo, &pango_layout);

            page_num1.borrow_mut().number+=1;
        });

        print_operation.set_allow_async(true);
        print_operation.connect_done(|_, _res| {
            //println!("printing done: {:?}", res);
        });

        print_operation
            .run(gtk::PrintOperationAction::PrintDialog, Some(&window))
            .expect("Couldn't print");
    }));

    exit.connect_activate(clone!(@weak window => move |_| {
        window.close();
    }));

    let css1 = Rc::clone(&css);

    zoom_in.connect_activate(move |_| {
        css1.borrow_mut().font += 2;
        load_css(css1.borrow_mut().font);
    });

    let css2 = Rc::clone(&css);

    zoom_out.connect_activate(move |_| {
        css2.borrow_mut().font -= 2;
        load_css(css2.borrow_mut().font);
    });

    let css3 = Rc::clone(&css);

    default_zoom.connect_activate(move |_| {
        css3.borrow_mut().font = 15;
        load_css(css3.borrow_mut().font);
    });

    about_prog.connect_activate(move |_| {
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
}
