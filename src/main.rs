extern crate rustbox;
extern crate time;
extern crate rand;

mod data;
mod rendering;
mod util;

use std::error::Error;
use std::default::Default;

use std::rc::Rc;
use std::cell::RefCell;

use rustbox::{Color, RustBox};
use rustbox::Key;

use time::Duration;

use data::user::User;
use data::avatar::Avatar;
use data::world::World;
use data::world::Tree;

use rendering::renderer_ascii::Representation;

use rand::Rng;

use util::io;

fn game_loop(rustbox: &RustBox, test_user: &mut User, test_world: &World){

	loop {
		rendering::renderer_ascii::clear(&rustbox);
		rustbox.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, test_user.get_name());
		rendering::renderer_ascii::render_world(&rustbox, 0, 1, rustbox.width(), rustbox.height(), &test_world);
		let pressed_key = match rustbox.peek_event(Duration::milliseconds(100), false) {
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
			Some(avatar) => {
				//avatar.borrow_mut().set_representation('X');
				match pressed_key{
					Some(Key::Up) => {avatar.borrow_mut().set_representation(Representation::new('^')); avatar.borrow_mut().move_up()},
					Some(Key::Down) => {avatar.borrow_mut().set_representation(Representation::new('v')); avatar.borrow_mut().move_down()},
					Some(Key::Left) => {avatar.borrow_mut().set_representation(Representation::new('<')); avatar.borrow_mut().move_left()},
					Some(Key::Right) => {avatar.borrow_mut().set_representation(Representation::new('>')); avatar.borrow_mut().move_right()},
					Some(_) => (),
					None =>()
				}
			},
			None => ()
		};

	}
}

fn main() {

	let rustbox = match RustBox::init(Default::default()) {
		Result::Ok(v) => v,
		Result::Err(e) => panic!("{}", e),
	};

	let mut name: String = String::new();
	rustbox.print(0, rustbox.height()-2, rustbox::RB_BOLD, Color::White, Color::Black, "Enter your name:");
	rustbox.present();
	io::read_into(0, rustbox.height()-1, &rustbox, &mut name);


	let mut world = World::new();

	let mut user = User::new(&name);

	user.set_avatar(Rc::new(RefCell::new(Avatar::new_with_representation(Representation::new('X')))));
	match user.get_avatar(){
		Some(avatar) =>	world.add_object(avatar),
		None => ()
	}

	let mut rng = rand::thread_rng();

	for _ in (0..rustbox.width()*2){
		let x=(rng.gen::<u32>()%(rustbox.width() as u32)) as i32;
		let y=(rng.gen::<u32>()%(rustbox.height() as u32)) as i32;
		for _ in (0..1) {
			world.add_object(Rc::new(RefCell::new(Tree::new(x+(rng.gen::<u32>()%6 as u32) as i32, y+(rng.gen::<u32>()%6 as u32) as i32, 0)) ));
		}
	}

	game_loop(&rustbox, &mut user, &world);





}
