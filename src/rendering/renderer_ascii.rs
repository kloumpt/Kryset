
use rustbox::{Color, RustBox};
use rustbox;

use data::world::World;

pub fn render_world(rustbox: &RustBox, x_begin: usize, y_begin: usize, x_end: usize, y_end: usize, world: &World){
	render_map(rustbox, x_begin, y_begin, x_end, y_end, world);

	let world_object_amount=world.get_object_amount();
	for n in (0..world_object_amount) {
		let test = world.get_object(n);
		match world.get_object(n){
			Some(an_element) => rustbox.print_char(x_begin+an_element.borrow().get_x() as usize, y_begin+an_element.borrow().get_y() as usize, rustbox::RB_BOLD, an_element.borrow().get_color(), Color::Black, an_element.borrow().get_representation()),
			None => ()
		}
	}
	rustbox.present();

}

pub fn render_map(rustbox: &RustBox, x_begin: usize, y_begin: usize, x_end: usize, y_end: usize, world: &World){
		let line=(0..(x_end-x_begin)).map(|_| ".").collect::<String>();
		for y in (y_begin..y_end) {
			//for x in (x_begin..x_end) {
				rustbox.print(x_begin, y, rustbox::RB_BOLD, Color::White, Color::Black, &line);
			//}
		}
		//rustbox.present();

}

pub fn clear(rustbox: &RustBox){
	rustbox.clear();
	//rustbox.present();

}
