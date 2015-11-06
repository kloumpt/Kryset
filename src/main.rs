extern crate rustbox;
extern crate time;

mod data;
mod rendering;
mod util;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use time::Duration;

use data::user::User;
use data::avatar::Avatar;
use data::world::World;

use util::io;


fn main() {

	let rustbox = match RustBox::init(Default::default()) {
		Result::Ok(v) => v,
		Result::Err(e) => panic!("{}", e),
	};

	let mut name: String = String::new();
	let mut test_user: User;
	let mut test_world: World=World::new();

	rustbox.print(0, rustbox.height()-2, rustbox::RB_BOLD, Color::White, Color::Black, "Enter your name:");
	rustbox.present();
	io::read_into(0, rustbox.height()-1, &rustbox, &mut name);

	let avatar_id =	test_world.add_object(Avatar::new_with_representation('X'));
	test_user = User::new(&name);
	test_user.set_avatar(avatar_id);


    //rustbox.print(0, rustbox.height()-1, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'q' to quit.");




	loop {
		rendering::renderer_ascii::clear(&rustbox);
		rustbox.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, test_user.get_name());
		rendering::renderer_ascii::render_world(&rustbox, 0, 1, rustbox.width(), rustbox.height(), &mut test_world);
		let pressed_key = match rustbox.peek_event(Duration::milliseconds(10), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; },
					Some(Key::Esc) =>{break;},
	                Some(v) => Some(v),
                    _ => {None}
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => {None}
        };

		match test_user.get_avatar() {
			&mut Some(avatar_id) => {
				match test_world.get_object(avatar_id) {
					Some(avatar) =>
						match pressed_key{
							Some(Key::Up) => avatar.move_up(),
							Some(Key::Down) => avatar.move_down(),
							Some(Key::Left) => avatar.move_left(),
							Some(Key::Right) => avatar.move_right(),
							Some(_) => (),
							None =>()
						},
					None => ()
				}
			}
			_ => ()
		}

    }
}
