extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use rand::{thread_rng, Rng};

use gtk::{Application, ApplicationWindow, Entry, Button};

fn main() {
    let application = Application::new(
        Some("co.dothq.gtk-demo"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    let mut rng = thread_rng();
    let num: i32 = rng.gen_range(0, 100);

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("App");
        window.set_default_size(350, 70);

        let win_title = gtk::Label::new(None);
        win_title.set_markup("<big>I am thinking of a number from 1-100, try guess what it is.</big>");

        let error_block = gtk::Label::new(None);
        error_block.hide();

        let button = Button::with_label("🎲 Guess 🎲");
        let guessing_field = Entry::new();

        guessing_field.set_margin_top(10);

        let row = gtk::Box::new(gtk::Orientation::Vertical, 5);
        row.add(&win_title);
        row.add(&guessing_field);
        row.add(&button);
        row.add(&error_block);

        button.connect_clicked(move |_| {
            error_block.set_markup("");
            error_block.hide();

            let guess = &guessing_field.get_text();
            
            if guess.chars().count() == 0 { 
                return 
            } else if guess.parse::<f64>().is_ok() {
                let guess: i32 = guess.trim().parse()
                .expect("please give me correct string number!");

                let mut rng = thread_rng();
                let chance = rng.gen_range(0.0, 1.0);

                if guess == num {
                    let to = format!("<big>🎉 You got it correct! It was {}!</big>", guess);
                    error_block.set_markup(&to);
                    error_block.show();
                } else if guess == 42 && chance < 0.1 {
                    /* 42 */
                    error_block.set_markup("<big>🌌 The answer to life, the universe and everything.</big>");
                    error_block.show();
                } else if guess < 1 {
                    error_block.set_markup("<big>🤦 You're way off, try guessing above zero.</big>");
                    error_block.show();
                } else if guess > 100 {
                    error_block.set_markup("<big>🤦 You're way off, try guessing below one hundred.</big>");
                    error_block.show();   
                } else {
                    let value = if guess > num { "lower 👇" } else { "higher 👆" };

                    let to = format!("<big>❌ Incorrect. It's actually {}</big>", value);
                    error_block.set_markup(&to);
                    error_block.show();
                }

            } else {
                let to = format!("<big>🚫 Error: {} is not a number.</big>", guess);
                error_block.set_markup(&to);
                error_block.show();
            }
        });
    
        row.set_property_margin(5);
        window.add(&row);

        window.show_all();
    });

    application.run(&[]);
}