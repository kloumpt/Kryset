extern crate rustbox;

mod data;
mod util;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;


use data::user::User;
use util::io;


fn main() {
	let rustbox = match RustBox::init(Default::default()) {
		Result::Ok(v) => v,
		Result::Err(e) => panic!("{}", e),
	};
	let mut name: String= String::new();
	io::read_into(1, 1, &rustbox, &mut name);
	let test_user: User = User::new(&name);


    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, &format!{"{}{}{}","Hello, ", &test_user.get_name(),"!"});
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}
