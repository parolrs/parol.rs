mod utils;

use utils::*;

fn main() {
    /* Init */
    match init() {
        Ok(_) => println!("Gtk initialized."),
        Err(err) => panic!("Gtk failed to initialize: {:?}", err)
    }

    /* Init window */
    let window = Window::new(WindowType::Toplevel);
    window.set_size_request(800, 400);
    window.set_resizable(false);
    window.set_icon(Some(&utils::bytes_to_pixbuf(include_bytes!("../ressources/parol.rs.png"))));

    /* Init buttons */
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

    /* Init box */
    let file_box = Box::new(Orientation::Horizontal, 5);
    file_box.add(&btn_new);
    file_box.add(&btn_load);
    file_box.add(&btn_save);
    file_box.add(&btn_close);

    let about_box = Box::new(Orientation::Horizontal, 5);
    about_box.add(&btn_about_me);
    about_box.add(&btn_about_parolrs);

    /* Init HeaderBar */
    let header_bar = HeaderBar::new();
    header_bar.set_show_close_button(true);
    header_bar.set_title("parol.rs");
    header_bar.set_decoration_layout("menu:close");

    header_bar.pack_start(&file_box);
    header_bar.pack_end(&about_box);

    window.set_titlebar(Some(&header_bar));

    /* Init ListStore */
    let list_store = ListStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    /* Init TreeView */
    let tree_view = init_view(&list_store);
    tree_view.set_model(Some(&list_store));

    /* Init ScrolledWindow */
    let scrolled_window = ScrolledWindow::new(None, None);
    scrolled_window.add(&tree_view);
    window.add(&scrolled_window);

    /* Events */
    window.connect_delete_event(|_, _| {
        main_quit();
        Inhibit(false)
    });

    btn_new.connect_clicked(clone!(list_store, btn_new, btn_load, btn_save, btn_close => move |_| {
        insert_row(&list_store, ["parol.rs is", "the", "best", "password manager !"]);

        /* Buttons */
        btn_new.set_sensitive(false);
        btn_load.set_sensitive(false);
        btn_save.set_sensitive(true);
        btn_close.set_sensitive(true);
    }));

    btn_load.connect_clicked(clone!(window, list_store, btn_new, btn_load, btn_save, btn_close => move |_| {
        let password = ask_password(&window, "Input database password");

        let parols = match parolrs::utils::load_database(&password) {
            Ok(parols) => parols,
            Err(err) => panic!("{}", err),
        };

        for i in 0 .. parols.len() {
            let application = parols.get(i).unwrap().get_application();
            let username    = parols.get(i).unwrap().get_username();
            let password    = parols.get(i).unwrap().get_password();
            let notes       = parols.get(i).unwrap().get_notes();
            insert_row(&list_store, [&application, &username, &password, &notes]);
        }

        /* Buttons */
        btn_new.set_sensitive(false);
        btn_load.set_sensitive(false);
        btn_save.set_sensitive(true);
        btn_close.set_sensitive(true);
    }));

    btn_save.connect_clicked(clone!(window, list_store, btn_new, btn_load, btn_save, btn_close => move |_| {
        let password = ask_password(&window, "Input password for the database");
        let parols = liststore_to_parols(&list_store);
        match parolrs::utils::save_database(&parols, &password) {
            Ok(_) => {
                let message = message_box(&window, MessageType::Info, ButtonsType::Ok, "Database saved !");
                message.run();
                message.destroy();
            },
            Err(err) => panic!("{}", err),
        }

        /* Buttons */
        btn_new.set_sensitive(false);
        btn_load.set_sensitive(false);
        btn_save.set_sensitive(true);
        btn_close.set_sensitive(true);
    }));

    btn_close.connect_clicked(clone!(list_store, btn_new, btn_load, btn_save, btn_close => move |_| {
        list_store.clear();

        /*
            Warning if model is not empty à faire
        */

        /* Buttons */
        btn_new.set_sensitive(true);
        btn_load.set_sensitive(true);
        btn_save.set_sensitive(false);
        btn_close.set_sensitive(false);
    }));

    btn_about_me.connect_clicked(clone!(window => move |_| {
        let message = message_box(&window, MessageType::Info, ButtonsType::Ok, "Ogromny, 19 years old, Russo-French Coder.");
        message.run();
        message.destroy();
    }));

    btn_about_parolrs.connect_clicked(clone!(window => move |_| {
        let message = message_box(&window, MessageType::Info, ButtonsType::Ok, "A strong password manager in coded in Rust with libsodium ❤️.");
        message.run();
        message.destroy();
    }));

    tree_view.connect_button_press_event(clone!(list_store, tree_view => move |_, event| {
        /* Right click */
        if event.get_button() == 3 {
            /* Add row */
            let add_row_hbox  = Box::new(Orientation::Horizontal, 0);
            let add_row       = MenuItem::new();
            let add_row_label = Label::new("Add row");
            let add_row_image = Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/add_row.png"))));

            add_row_hbox.add(&add_row_image);
            add_row_hbox.add(&add_row_label);

            add_row.add(&add_row_hbox);

            /* Event */
            add_row.connect_activate(clone!(list_store => move |_| {
                insert_row(&list_store, [""; 4]);
            }));

            /* Remove row */
            let remove_row_hbox  = Box::new(Orientation::Horizontal, 0);
            let remove_row       = MenuItem::new();
            let remove_row_label = Label::new("Remove row");
            let remove_row_image = Image::new_from_pixbuf(Some(&bytes_to_pixbuf(include_bytes!("../ressources/remove_row.png"))));

            remove_row_hbox.add(&remove_row_image);
            remove_row_hbox.add(&remove_row_label);

            remove_row.add(&remove_row_hbox);

            /* Event */
            remove_row.connect_activate(clone!(list_store, tree_view => move |_| {
                let (paths, _) = tree_view.get_selection().get_selected_rows();
                for path in paths.iter().rev() {
                    list_store.remove(&match list_store.get_iter(&path) {
                        Some(iter) => iter,
                        None => continue,
                    });
                }
            }));

            /* Menu */
            let menu = Menu::new();

            menu.add(&add_row);
            menu.add(&remove_row);

            menu.show_all();
            menu.popup_easy(event.get_button(), event.get_time());
        }

        Inhibit(false)
    }));

    /* Run */
    window.show_all();
    utils::main();
}