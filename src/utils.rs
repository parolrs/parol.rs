#![macro_use]

pub extern crate gtk;
pub extern crate gdk_pixbuf;
pub extern crate parolrs;

pub use self::gdk_pixbuf::{Pixbuf, PixbufLoader};
pub use self::gtk::prelude::*;
pub use self::gtk::{
    init, main_quit, main,
    Window, WindowType, HeaderBar, Button, Image, Box, Orientation, MessageDialog, MessageType,
    ButtonsType, DialogFlags, TreeView, TreeViewColumn, CellRendererText, TreeViewGridLines,
    ListStore, Entry, TreePath, Menu, MenuItem, Label, SelectionMode, ScrolledWindow
};

const COLUMN_NAME: [&'static str; 4] = ["Program", "Username", "Password", "Note"];

macro_rules! clone {
    (@param _) =>
        (_);
    (@param $x:ident) =>
        ($x);
    ($($n:ident),+ => move || $body:expr) =>
        ({
            $(let $n = $n.clone();)+
            move || $body
        });
    ($($n:ident),+ => move |$($p:tt),+| $body:expr)
        => ({
            $(let $n = $n.clone();)+
            move |$(clone!(@param $p),)+| $body
        });
}

pub fn message_box(window: &Window, message_type: MessageType, button: ButtonsType, message: &str) -> MessageDialog {
    MessageDialog::new(Some(window), DialogFlags::all(), message_type, button, message)
}

pub fn ask_password(window: &Window, message: &str) -> String {
    let input_password = MessageDialog::new(Some(window), DialogFlags::all(), MessageType::Question, ButtonsType::OkCancel, message);
    let entry = Entry::new();

    input_password.get_content_area().pack_start(&entry, true, false, 0);
    input_password.show_all();
    input_password.set_size_request(200, 100);

    if input_password.run() != -5 {
        panic!();
    }

    if let Some(password) = entry.get_text() {
        input_password.destroy();
        return password;
    } else {
        panic!()
    }
}

pub fn bytes_to_pixbuf(bytes: &[u8]) -> Pixbuf {
    let pixbuf = match PixbufLoader::new_with_type("png") {
        Ok(pixbuf) => pixbuf,
        Err(e) => panic!("{}", e),
    };

    if let Err(e) = pixbuf.loader_write(bytes) {
        panic!("{}", e);
    } else {
        if let Err(e) = pixbuf.close() {
            panic!("{}", e);
        }
    }

    if let Some(pixbuf) = pixbuf.get_pixbuf() {
        return pixbuf;
    } else {
        panic!("No pixbuf !");
    }
}

pub fn init_view(list_store: &ListStore) -> TreeView {
    let treeview = TreeView::new();

    for i in 0 .. 4 {
        let cell   = CellRendererText::new();
        cell.set_alignment(0.5, 0.5);
        cell.set_property_editable(true);
        cell.connect_edited(clone!(list_store => move |_, tree_path, new_text| {
            if let Some(iter) = list_store.get_iter(&tree_path) {
                list_store.set_value(&iter, i, &new_text.to_value());
            }
        }));

        let column = TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title(COLUMN_NAME[i as usize]);
        column.add_attribute(&cell, "text", i as i32);
        column.set_resizable(true);
        column.set_alignment(0.5);
        column.set_min_width(match i { 0 => 200, 1 => 160, 2 => 180, _ => -1 });
        column.set_clickable(match i { 0...2 => true, _ => false });

        treeview.append_column(&column);
    }

    treeview.set_grid_lines(TreeViewGridLines::Both);
    treeview.set_fixed_height_mode(true);
    treeview.get_selection().set_mode(SelectionMode::Multiple);

    return treeview;
}

pub fn insert_row(model: &ListStore, data: [&str; 4]) {
    let iter = model.append();
    for i in 0 .. 4 {
        model.set_value(&iter, i, &data[i as usize].to_value());
    }
}

pub fn liststore_to_parols(model: &ListStore) -> parolrs::core::Parols {
    let mut parols = parolrs::core::Parols::new();

    if let Some(iter) = model.get_iter_first() {
        loop {
            let mut data = Vec::with_capacity(4);
            for i in 0..4 {
                data.push(model.get_value(&iter, i as i32).get::<String>().unwrap());
            }

            parols.push(
                parolrs::core::Parol::new_with_arguments(
                    &data[0], &data[1], &data[2], &data[3]
                )
            );

            if model.iter_next(&iter) == false { break; }
        }
    }

    return parols
}