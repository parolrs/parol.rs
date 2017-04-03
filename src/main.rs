mod utils;

use utils::*;

fn main() {
    match utils::init() {
        Ok(_) => println!("Gtk initialized."),
        Err(err) => panic!("Gtk failed to initialize: {:?}", err)
    }

    /*
        Init window
    */
    let window = Window::new(WindowType::Toplevel);
    window.set_size_request(800, 400);
    window.set_resizable(false);
    window.set_icon(Some(&utils::bytes_to_pixbuf(include_bytes!("../ressources/parol.rs.png"))));

    /*
        Button
    */
    let btn_new = Button::new();
    btn_new.set_tooltip_text("New");
    btn_new.add(&Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/new.png")))));

    let btn_load = Button::new();
    btn_load.set_tooltip_text("Load");
    btn_load.add(&Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/load.png")))));

    let btn_save = Button::new();
    btn_save.set_tooltip_text("Save");
    btn_save.add(&Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/save.png")))));

    let btn_close = Button::new();
    btn_close.set_tooltip_text("Close");
    btn_close.add(&Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/close.png")))));

    let btn_about_me = Button::new();
    btn_about_me.set_tooltip_text("About me");
    btn_about_me.add(&Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/about_me.png")))));

    let btn_about_parolrs = Button::new();
    btn_about_parolrs.set_tooltip_text("About parol.rs");
    btn_about_parolrs.add(&Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/parol.rs.png")))));

    btn_save.set_sensitive(false);
    btn_close.set_sensitive(false);

    /*
        Box
    */
    let file_box = Box::new(Orientation::Horizontal, 5);
    file_box.add(&btn_new);
    file_box.add(&btn_load);
    file_box.add(&btn_save);
    file_box.add(&btn_close);

    let about_box = Box::new(Orientation::Horizontal, 5);
    about_box.add(&btn_about_me);
    about_box.add(&btn_about_parolrs);

    /*
        HeaderBar
    */
    let header_bar = HeaderBar::new();
    header_bar.set_show_close_button(true);
    header_bar.set_title("parol.rs");
    header_bar.set_decoration_layout("menu:close");

    header_bar.pack_start(&file_box);
    header_bar.pack_end(&about_box);

    /*
        Model
    */
    let listmodel = ListStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    /*
        Treeview
    */
    let treeview = init_view();
    treeview.set_model(Some(&listmodel));

    /*
        ScrolledWindow
    */
    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.add(&treeview);

    /*
        Events
    */
    let model = listmodel.clone();
    let _btn_new = btn_new.clone();
    let _btn_load = btn_load.clone();
    let _btn_save = btn_save.clone();
    let _btn_close = btn_close.clone();
    btn_new.connect_clicked(move |_| {
        insert_row(&model, ["parol.rs is", "the", "best", "password manager !"]);

        /* Btn */
        _btn_new.set_sensitive(false);
        _btn_load.set_sensitive(false);
        _btn_save.set_sensitive(true);
        _btn_close.set_sensitive(true);
    });

    let w = window.clone();
    let model = listmodel.clone();
    let _btn_new = btn_new.clone();
    let _btn_load = btn_load.clone();
    let _btn_save = btn_save.clone();
    let _btn_close = btn_close.clone();
    btn_load.connect_clicked(move |_| {
        let password = {
            let input_password = MessageDialog::new(
                Some(&w),
                DialogFlags::all(),
                MessageType::Question,
                ButtonsType::OkCancel,
                "Input database password"
            );
            let entry = Entry::new();
            input_password.get_content_area().pack_start(&entry, true, false, 0);
            input_password.show_all();
            input_password.set_size_request(200, 100);
            input_password.run();
            let password = entry.get_text();
            input_password.destroy();
            match password {
                Some(password) => password,
                None => String::from(""),
            }
        };

        let parols = match parolrs::utils::load_database(&password) {
            Ok(parols) => parols,
            Err(err) => panic!("{}", err),
        };

        for i in 0 .. parols.len() {
            let application = parols.get(i).unwrap().get_application();
            let username    = parols.get(i).unwrap().get_username();
            let password    = parols.get(i).unwrap().get_password();
            let notes       = parols.get(i).unwrap().get_notes();
            insert_row(&model, [&application, &username, &password, &notes]);
        }

        /* Btn */
        _btn_new.set_sensitive(false);
        _btn_load.set_sensitive(false);
        _btn_save.set_sensitive(true);
        _btn_close.set_sensitive(true);
    });

    let w = window.clone();
    let model = listmodel.clone();
    let _btn_new = btn_new.clone();
    let _btn_load = btn_load.clone();
    let _btn_save = btn_save.clone();
    let _btn_close = btn_close.clone();
    btn_save.connect_clicked(move |_| {
        let input_password = MessageDialog::new(
            Some(&w),
            DialogFlags::all(),
            MessageType::Question,
            ButtonsType::OkCancel,
            "Input password for the database"
        );
        let entry = Entry::new();

        input_password.get_content_area().pack_start(&entry, true, false, 0);
        input_password.show_all();
        input_password.set_size_request(200, 100);

        if input_password.run() == -5 {
            let password = entry.get_text();

            let password = match password {
                Some(password) => password,
                None => String::from(""),
            };

            let parols = liststore_to_parols(&model);
            match parolrs::utils::save_database(&parols, &password) {
                Ok(_) => {
                    let message = message_box(&w, MessageType::Info, ButtonsType::Ok, "Database saved !");
                    message.run();
                    message.destroy();
                },
                Err(err) => panic!("{}", err),
            }

            /* Btn */
            _btn_new.set_sensitive(false);
            _btn_load.set_sensitive(false);
            _btn_save.set_sensitive(true);
            _btn_close.set_sensitive(true);
        }

        input_password.destroy();
    });

    let w = window.clone();
    let model = listmodel.clone();
    let _btn_new = btn_new.clone();
    let _btn_load = btn_load.clone();
    let _btn_save = btn_save.clone();
    let _btn_close = btn_close.clone();
    btn_close.connect_clicked(move |_| {
        model.clear();

        /*
            Warning if model is not empty à faire
        */

        /* Btn */
        _btn_new.set_sensitive(true);
        _btn_load.set_sensitive(true);
        _btn_save.set_sensitive(false);
        _btn_close.set_sensitive(false);
    });

    let w = window.clone();
    btn_about_me.connect_clicked(move |_| {
        let message = message_box(&w, MessageType::Info, ButtonsType::Ok, "Ogromny, 19 years old, Russo-French Coder.");
        message.run();
        message.destroy();
    });

    let w = window.clone();
    btn_about_parolrs.connect_clicked(move |_| {
        let message = message_box(&w, MessageType::Info, ButtonsType::Ok, "A strong password manager in coded in Rust with libsodium ❤️.");
        message.run();
        message.destroy();
    });

    let model = listmodel.clone();
    treeview.connect_button_press_event(move |tree, event| {
        // Right click
        if event.get_button() == 3 {
             /*
                Add row
            */

            let add_row       = MenuItem::new();
            let add_row_hbox  = Box::new(Orientation::Horizontal, 0);
            let add_row_label = Label::new("Add row");
            let add_row_image = Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/add_row.png"))));

            add_row_hbox.add(&add_row_image);
            add_row_hbox.add(&add_row_label);

            add_row.add(&add_row_hbox);

            /*
                Remove row
            */

            let remove_row       = MenuItem::new();
            let remove_row_hbox  = Box::new(Orientation::Horizontal, 0);
            let remove_row_label = Label::new("Remove row");
            let remove_row_image = Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/remove_row.png"))));

            remove_row_hbox.add(&remove_row_image);
            remove_row_hbox.add(&remove_row_label);

            remove_row.add(&remove_row_hbox);

            /*
                Events
            */
            let model = model.clone();
            add_row.connect_activate(move |_| {
                insert_row(&model, ["", "", "", ""]);
            });

            remove_row.connect_activate(|_| {
                println!("");
            });

            /*
                Menu
            */

            let menu = Menu::new();

            menu.add(&add_row);
            menu.add(&remove_row);

            menu.show_all();
            menu.popup_easy(event.get_button(), event.get_time());
        }
        Inhibit(false)
    });

    window.connect_delete_event(|_, _| {
        utils::main_quit();
        Inhibit(false)
    });

    /*
        Finalize window
    */
    window.set_titlebar(Some(&header_bar));
    window.add(&scrolled_window);
    window.show_all();

    utils::main();
}