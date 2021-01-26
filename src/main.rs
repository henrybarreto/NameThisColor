use std::{cell::RefCell, path::Path, rc::Rc};

use gtk::prelude::*;
use gio::prelude::*;
use ntc::Color;

mod ntc;

fn main() {
    gtk::init().expect("Could not init the GTK");// This function will init the gtk

    let application = gtk::Application::new(
        Some("dev.henrybarreto.name-this-color"),
        Default::default()
    ).expect("Could not create the gtk aplication");
    let builder: gtk::Builder =  gtk::Builder::from_file(Path::new("ntc.glade"));

    let colors_saved = Rc::new(RefCell::new(ntc::NTC::new()));

    application.connect_activate(move |_| {
        let main_window: gtk::Window = builder.get_object("main_window").expect("Could not get the object main_window");
        let save_button: gtk::Button = builder.get_object("save_button").expect("Could not get the save_button");
        let color_selection: gtk::ColorButton = builder.get_object("color_selection").expect("Could not get the color_selection");
        let color_name_entry: gtk::Entry = builder.get_object("color_name_entry").expect("Could not get the color_name_entry");
        //let _select_color_label: gtk::Label = builder.get_object("select_color_label").expect("Could not get the select_color_label");
        //let _name_color_label: gtk::Label = builder.get_object("name_color_label").expect("Could not get the name_color_label");
        let registered_color_label: gtk::Label = builder.get_object("registered_color_label").expect("Could not get the registred_color_label");

        let colors_saved = colors_saved.clone();
        save_button.connect_clicked(move |_| {
            let color_rgba = color_selection.get_rgba();
            let color: Color = Color {
                red: color_rgba.red,
                green: color_rgba.green,
                blue: color_rgba.blue,
                alpha: color_rgba.alpha
            };
            let name = color_name_entry.get_text().to_string();

            registered_color_label.set_visible(true);
            if let Ok(()) = colors_saved.borrow_mut().save_color(color, name) {
                registered_color_label.set_text("Registered!");
            } else {
                registered_color_label.set_text("Already Registered!");
            }
        });

        main_window.connect_destroy(move |_| {
            gtk::main_quit();
        });

        main_window.show();
    });

    application.run(&[]);
    gtk::main();
}
