
use rustbox::{Color, RustBox};
use rustbox;

use data::world::World;

pub struct Representation{
	characters: Vec<(i32, i32, char)>
}

impl Representation{
	pub fn new(character: char) -> Representation{
		Representation{characters: vec![(0, 0, character)]}
	}

		pub fn new_composed(character: Vec<(i32, i32, char)>) -> Representation{
			Representation{characters: character}
		}

	pub fn get_character(&self, index: usize) -> Option<(i32, i32, char)>{
		if index >= self.characters.len() {
			return None
		}
		Some(self.characters[index].clone())
	}
	pub fn get_character_amount(&self) -> usize{
		self.characters.len()
	}
}

pub fn render_world(rustbox: &RustBox, x_begin: usize, y_begin: usize, x_end: usize, y_end: usize, world: &World){
	render_map(rustbox, x_begin, y_begin, x_end, y_end, world);

	let world_object_amount=world.get_object_amount();
	for n in (0..world_object_amount) {
		match world.get_object(n){
			Some(an_element) => {

				let characters_amount=an_element.borrow().get_representation().get_character_amount();
				for m in (0..characters_amount) {
					match an_element.borrow().get_representation().get_character(m) {

					Some((x_off, y_off, c)) => {
						rustbox.print_char(x_begin+(an_element.borrow().get_x()+x_off) as usize, y_begin+(an_element.borrow().get_y()+y_off) as usize, rustbox::RB_BOLD, an_element.borrow().get_color(), Color::Black, c)

					},
					None => ()
				}
				}
			},
			None => ()
		}

	}
	rustbox.present();

}

pub fn render_map(rustbox: &RustBox, x_begin: usize, y_begin: usize, x_end: usize, y_end: usize, _world: &World){
		let line=(0..(x_end-x_begin)).map(|_| " ").collect::<String>();
		for y in (y_begin..y_end) {
				rustbox.print(x_begin, y, rustbox::RB_BOLD, Color::Yellow, Color::Black, &line);
		}
		//rustbox.present();

}

pub fn clear(rustbox: &RustBox){
	rustbox.clear();
	//rustbox.present();

}
