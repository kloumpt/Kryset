extern crate rustbox;
extern crate time;
extern crate rand;
extern crate byteorder;
extern crate rustc_serialize;

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

use rand::Rng;

use data::user::User;
use data::avatar::Avatar;
use data::world::World;
use data::world::Tree;

use rendering::renderer_ascii::Representation;


use util::io;

use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::net::UdpSocket;
/*
use byteorder::ByteOrder;
use byteorder::BigEndian;
*/
use std::env;

use rustc_serialize::json::{ToJson, Json};


fn _accept_string(socket: &UdpSocket) -> String{
	let mut buf = [0; 1024];

	let (amt, _src) = match socket.recv_from(&mut buf) {
		Ok(v) => v,
		Err(e) => panic!("error : {}", e)
	};
	match std::str::from_utf8(&buf[..amt]){
		Ok(v) => v.to_string(),
		Err(_) => "".to_string()
	}
}


pub fn render(rustbox: &RustBox, test_world: &World){
	rendering::renderer_ascii::clear(&rustbox, 0, 1, rustbox.width(), rustbox.height());
	rendering::renderer_ascii::render_world(&rustbox, 0, 1, rustbox.width(), rustbox.height(), &test_world);

}

pub fn accept_input(rustbox: &RustBox) -> Option<Key>{
	match rustbox.peek_event(Duration::milliseconds(10), false) {
		Ok(rustbox::Event::KeyEvent(key)) => {
			match key {
				Some(Key::Char('q')) => Some(Key::Esc),
				Some(Key::Esc) =>Some(Key::Esc),
				Some(v) => Some(v),
				_ => None
			}
		},
		Err(e) => panic!("{}", e.description()),
		_ => None
	}
}


fn manage_avatar(avatar: &Rc<RefCell<Avatar>>, pressed_key: &Option<Key>){
	match pressed_key{
		&Some(Key::Up) => {avatar.borrow_mut().set_representation(Representation::new('^')); avatar.borrow_mut().move_up()},
		&Some(Key::Down) => {avatar.borrow_mut().set_representation(Representation::new('v')); avatar.borrow_mut().move_down()},
		&Some(Key::Left) => {avatar.borrow_mut().set_representation(Representation::new('<')); avatar.borrow_mut().move_left()},
		&Some(Key::Right) => {avatar.borrow_mut().set_representation(Representation::new('>')); avatar.borrow_mut().move_right()},
		&Some(_) => (),
		&None =>()
	};
}


fn accept_user(socket: &UdpSocket) -> Option<User>{
	let mut buf = [0; 1024];

	let (amt, _src) = match socket.recv_from(&mut buf) {
		Ok(v) => v,
		Err(e) => panic!("error : {}", e)
	};
	match std::str::from_utf8(&buf[..amt]){
		Ok(v) => match Json::from_str(v){
			Ok(json_data)=>Some(User::from_json(&json_data)),
			Err(_) => None
		},
		Err(_) => panic!("tchelk")
	}
}

fn send_user(socket: &UdpSocket, user: &User) {
	let addr=SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127u8, 0u8, 0u8, 1u8), 8080));
	let user_data=user.to_json();
	match user_data.as_string(){
		Some(data) =>
			match socket.send_to(&data.to_string().into_bytes(), &addr){
				Ok(_) => (),
				Err(e) => panic!("error : {}", e)
			},
		None=> ()
	};
}



fn game_loop(rustbox: &RustBox, test_user: &mut User, test_world: &World, socket: &mut UdpSocket, server: bool){
	loop {
		render(&rustbox, &test_world);
		if server {
			rustbox.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Server | Connected user : {}", test_user.get_name()));
			rustbox.present();

		}else {
			match test_user.to_json(){
				Json::String(data)=>rustbox.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Client | User data : {}", &data)),
				_ => rustbox.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Client | User name : {}", test_user.get_name()))
			};
			rustbox.present();

			let pressed_key = accept_input(&rustbox);
			match &pressed_key{
				&Some(Key::Esc) => return,
				_ => ()
			};
			match test_user.get_avatar() {
				Some(avatar) => manage_avatar(&avatar, &pressed_key),
				None => ()
			};
		}

		match test_user.get_avatar() {
			Some(avatar) =>
				if server {
					match accept_user(&socket){
						Some(user) => {
							test_user.set_name(user.get_name().to_string());
							match user.get_avatar(){
								Some(foreign_avatar) =>{
									avatar.borrow_mut().set_x(foreign_avatar.borrow().get_x());
									avatar.borrow_mut().set_y(foreign_avatar.borrow().get_y());
									avatar.borrow_mut().set_representation(Representation::new_from(foreign_avatar.borrow().get_representation()));
								},
								None => ()
							}
						},
						None => ()
					};
				} else {
					send_user(socket, &test_user);
				},
			None => ()
		};

	}
}


fn main() {

	let mut server=false;
	let mut port=8081;
	for argument in env::args() {
	    if argument == "server"{
			server=true;
			port=8080;
		}
	}

	let rustbox = match RustBox::init(Default::default()) {
		Result::Ok(v) => v,
		Result::Err(e) => panic!("{}", e),
	};

	let mut name: String = String::new();
	if !server{
		rustbox.print(0, rustbox.height()-2, rustbox::RB_BOLD, Color::White, Color::Black, "Enter your name:");
		rustbox.present();
		io::read_into(0, rustbox.height()-1, &rustbox, &mut name);
	}

	let mut world = World::new();

	let mut user = User::new(&name);

	user.set_avatar(Rc::new(RefCell::new(Avatar::new_with_representation(Representation::new('X')))));
	match user.get_avatar(){
		Some(avatar) =>	world.add_object(avatar),
		None => ()
	}



	let mut rng = rand::thread_rng();

	for _ in (0..rustbox.width()*2){
		let x=(rng.gen::<u64>()%(rustbox.width() as u64)) as i64;
		let y=(rng.gen::<u64>()%(rustbox.height() as u64)) as i64;
		world.add_object(Rc::new(RefCell::new(Tree::new(x, y, 0)) ));
	}

	let adress = Ipv4Addr::new(127, 0, 0, 1);
	let mut socket = match UdpSocket::bind((adress, port)) {
		Ok(v) => v,
		Err(e) => panic!("error : {}", e)
	};
	game_loop(&rustbox, &mut user, &world, &mut socket, server);

	drop(socket);




}
