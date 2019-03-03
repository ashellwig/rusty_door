extern crate cursive;

use cursive::Cursive;
use cursive::views::{Dialog, TextView};

use std::process::Command;

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::around(
                    TextView::new("Select Window Manager or press <q> to quit"))
                    .title("Rusty Door")
                    .button("i3", |s| start_wm(s, "i3"))
                    .button("About", |s| s.add_layer(Dialog::info("Created with Rust by Ashton Hellwig using Cursive library in 2019"))));

    siv.run();
}

fn start_wm(s: &mut Cursive, _msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text("Continue")
        .title("Selected Window Manager")
        .button("Start", |_s| exec_xinit("i3"))
        .button("Quit", |s| s.quit()));
}

fn exec_xinit(wm: &str) {
    let start_command = match wm {
        "i3" => "i3",
        "xfce" | "xfce4" => "startxfce4",
        "kde" | "plasma" | "kde5" | "plasma5" => "startkde",
        "gnome" => "gnome-session",
        _ => "i3",
    };

    if start_command == "gnome-session" {
        Command::new("startx")  
            .env("GDK_BACKEND", "x11")
            .arg(&start_command)
            .spawn()
            .expect("Failed to start gnome");
    } else {
        Command::new("startx")
            .arg(&start_command)
            .spawn()
            .expect("Failed to start");
    }
}
