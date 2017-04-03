pub extern crate gtk;
pub extern crate gdk_pixbuf;
pub extern crate parolrs;

pub use std::collections::HashMap;

pub use self::gtk::prelude::*;
pub use self::gtk::{
    init, main_quit, main,
    Window, WindowType,
    HeaderBar,
    Button,
    Image,
    Box, Orientation,
    MessageDialog, MessageType,
    ButtonsType,
    DialogFlags,
    TreeView, TreeViewColumn,
    CellRendererText,
    TreeViewGridLines,
    ListStore,
    Entry,
    TreePath
};
pub use self::gdk_pixbuf::{
    Pixbuf, PixbufLoader
};

pub fn message_box(window: &Window, message_type: MessageType, button: ButtonsType, message: &str) -> MessageDialog {
    MessageDialog::new(
        Some(window),
        DialogFlags::all(),
        message_type,
        button,
        message
    )
}

pub fn bytes_to_pixbuf(bytes: &[u8]) -> Pixbuf {
    match PixbufLoader::new_with_type("png") {
        Ok(p) => {
            match p.loader_write(bytes) {
                Ok(_) => {
                    match p.close() {
                        Ok(_) => {
                            match p.get_pixbuf() {
                                Some(pixbuf) => pixbuf,
                                None => panic!("No pixbuf !"),
                            }
                        },
                        Err(e) => panic!("{}", e),
                    }
                },
                Err(e) => panic!("{}", e),
            }
        },
        Err(e) => panic!("{}", e),
    }
}

pub fn init_view() -> TreeView {
    let column_name = ["Program", "Username", "Password", "Note"];
    let treeview = TreeView::new();

    for i in 0 .. 4 {
        let column = TreeViewColumn::new();
        let cell   = CellRendererText::new();
        cell.set_alignment(0.5, 0.5);

        column.pack_start(&cell, true);
        column.set_title(column_name[i]);
        column.add_attribute(&cell, "text", i as i32);
        column.set_min_width(match i {
            0 => 200,
            1 => 160,
            2 => 180,
            _ => -1,
        });
        column.set_resizable(true);
        column.set_alignment(0.5);
        column.set_clickable(match i {
            0 => true,
            1 => true,
            2 => true,
            _ => false,
        });

        treeview.append_column(&column);
    }

    treeview.set_grid_lines(TreeViewGridLines::Vertical);
    treeview.set_fixed_height_mode(true);

    treeview
}

pub fn insert_row(model: &ListStore, data: [&str; 4]) {
    let iter = model.append();
    model.set_value(&iter, 0, &data[0].to_value());
    model.set_value(&iter, 1, &data[1].to_value());
    model.set_value(&iter, 2, &data[2].to_value());
    model.set_value(&iter, 3, &data[3].to_value());
}

pub fn liststore_to_parols(model: &ListStore) -> parolrs::core::Parols {
    let iter = match model.get_iter_first() {
        Some(iter) => iter,
        None => return parolrs::core::Parols::new(), // empty database
    };
    let mut parols = parolrs::core::Parols::new();

    loop {
        let application = match model.get_value(&iter, 0).get::<String>() {
            Some(string) => string,
            None => panic!("Cannot convert to String !"),
        };
        let username    = match model.get_value(&iter, 1).get::<String>() {
            Some(string) => string,
            None => panic!("Cannot convert to String !"),
        };
        let password    = match model.get_value(&iter, 2).get::<String>() {
            Some(string) => string,
            None => panic!("Cannot convert to String !"),
        };
        let notes       = match model.get_value(&iter, 3).get::<String>() {
            Some(string) => string,
            None => panic!("Cannot convert to String !"),
        };

        let parol = parolrs::core::Parol::new_with_arguments(
            &application,
            &username,
            &password,
            &notes,
        );

        parols.push(parol);

        if model.iter_next(&iter) == false {
            break;
        }
    }

    return parols
}