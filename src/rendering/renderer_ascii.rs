
use rustbox::{Color, RustBox};
use rustbox;

use rustc_serialize::json::{ToJson, Json};
use data::world::World;

pub struct Representation{
	characters: Vec<(i64, i64, char)>
}

impl Representation{
	pub fn new(character: char) -> Representation{
		Representation{characters: vec![(0, 0, character)]}
	}

	pub fn new_composed(characters: Vec<(i64, i64, char)>) -> Representation{
		Representation{characters: characters}
	}
	pub fn new_from(representation: &Representation) -> Representation{
		let mut characters: Vec<(i64, i64, char)>=Vec::new();
		for i in (0..representation.get_character_amount()){
			match representation.get_character(i){
				Some(tuple) => characters.push(tuple),
				None => ()
			};
		}
		Representation{characters: characters}
	}

	pub fn from_json(data: &Json)->Representation{
		let mut x=1i64;
		let mut y=1i64;
		let mut character=' ';
		let mut characters: Vec<(i64, i64, char)>=Vec::new();

	    match data.as_object(){
			Some(obj) =>{
		        match &obj.get("characters") {
		            &Some(v) => match v.clone(){
						Json::Array(v) =>{
							for tuple in v{

									    match tuple.as_object(){
											Some(obj) => characters.push((obj.get("x").unwrap().as_i64().unwrap(), obj.get("x").unwrap().as_i64().unwrap(), obj.get("char").unwrap().as_string().unwrap().to_string().as_bytes()[0] as char)),
											None => ()
										};
							};
							()
						},
						_ =>()
					},
					&None => ()
				}
			},
				/*
		        match obj.get("y") {
		            Some(&Json::I64(v)) => {y=v; ()},
		            Some(&Json::U64(v)) => {y=v as i64; ()},
					_=>()
				};
		        match obj.get("z") {
		            Some(&Json::I64(v)) => {z=v; ()},
		            Some(&Json::U64(v)) => {z=v as i64; ()},
					_=>()
				};

		        match obj.get("representation") {
		            Some(v) => {representation=Some(Representation::from_json(v)); ()},
					_=>()
				};*/
			None =>()
		};

		Representation::new_composed(characters)
	}

	pub fn get_character(&self, index: usize) -> Option<(i64, i64, char)>{
		if index >= self.characters.len() {
			return None
		}
		Some(self.characters[index].clone())
	}
	pub fn get_character_amount(&self) -> usize{
		self.characters.len()
	}
	pub fn to_json(&self) -> Json {
		let mut result="{\"characters\": [".to_string();
		for &(x, y, char) in &self.characters{
			result.push_str(&format!("{{\"x\": {}, \"y\": {}, \"char\": \"{}\"}}", x, y, char));
		}
		result.push_str("]}");
		Json::String(result)
    }
}

impl ToJson for Representation {
    fn to_json(&self) -> Json {
		self.to_json()
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

pub fn render_map(_rustbox: &RustBox, _x_begin: usize, _y_begin: usize, _x_end: usize, _y_end: usize, _world: &World){
		/*let line=(0..(x_end-x_begin)).map(|_| " ").collect::<String>();
		for y in (y_begin..y_end) {
				rustbox.print(x_begin, y, rustbox::RB_BOLD, Color::Yellow, Color::Black, &line);
		}*/
		//rustbox.present();

}

pub fn clear(rustbox: &RustBox, x_begin: usize, y_begin: usize, x_end: usize, y_end: usize){
		let line=(0..(x_end-x_begin)).map(|_| " ").collect::<String>();
		for y in (y_begin..y_end) {
				rustbox.print(x_begin, y, rustbox::RB_BOLD, Color::Black, Color::Black, &line);
		}
}
