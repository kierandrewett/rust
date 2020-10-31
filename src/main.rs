extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Entry, Button};

fn main() {
    let application = Application::new(
        Some("co.dothq.gtk-demo"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("App");
        window.set_default_size(350, 70);

        let win_title = gtk::Label::new(None);
        win_title.set_markup("<big>I am thinking of a number from 1-100, try guess what it is.</big>");

        let button = Button::with_label("Click me!");
        let guessing_field = Entry::new();

        guessing_field.set_margin_top(10);

        let row = gtk::Box::new(gtk::Orientation::Vertical, 5);
        row.add(&win_title);
        row.add(&guessing_field);
        row.add(&button);

        button.connect_clicked(move |_| {
            let guess = &guessing_field.get_text();
            
            if guess.parse::<f64>().is_ok() {
                println!("that is a number")
            } else {
                println!("not a number")
            }
        });
    
        row.set_property_margin(5);
        window.add(&row);

        window.show_all();
    });

    application.run(&[]);
}
