use data::world::Element;
use rustbox::Color;
use rendering::renderer_ascii::Representation;
use rustc_serialize::json::{ToJson, Json};

pub struct Avatar{
	x: i64,
	y: i64,
	z: i64,
	representation: Representation
}

impl Avatar{
	pub fn _new() -> Avatar{
		Avatar{x: 0, y: 0, z: 0, representation: Representation::new('X')}
	}

	pub fn new_with_representation(representation: Representation) -> Avatar{
		Avatar{x: 0, y: 0, z: 0, representation: representation}
	}


	pub fn from_json(data: &Json)->Avatar{
		let mut x=1i64;
		let mut y=1i64;
		let mut z=1i64;
		let mut representation=Representation::new('X');

	    match data.as_object(){
			Some(obj) =>{
		        match obj.get("x") {
		            Some(&Json::I64(v)) => {x=v; ()},
		            Some(&Json::U64(v)) => {x=v as i64; ()},
					_=>()
				};
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
		            Some(v) => {representation=Representation::from_json(v); ()},
					_=>()
				};
			},
			None =>()
		};
		Avatar{x: x, y: y, z: z, representation: representation}
	}

	pub fn move_up(&mut self){
		self.y-=1;
	}
	pub fn move_down(&mut self){
		self.y+=1;
	}
	pub fn move_left(&mut self){
		self.x-=1;
	}
	pub fn move_right(&mut self){
		self.x+=1;
	}
	pub fn get_x(&self) -> i64{
		self.x
	}
	pub fn get_y(&self) -> i64{
		self.y
	}

	pub fn get_z(&self) -> i64{
		self.z
	}


	pub fn set_x(&mut self, x: i64){
		self.x = x;
	}

	pub fn set_y(&mut self, y: i64){
		self.y = y;

	}

	pub fn set_z(&mut self, z: i64){
		self.z = z;

	}

	pub fn get_representation(&self) -> &Representation{
		&self.representation
	}

	pub fn set_representation(&mut self, representation: Representation){
		self.representation=representation
	}
	fn get_color(&self) -> Color{
		Color::Red
	}

    pub fn to_json(&self) -> Json {
		match self.representation.to_json(){
			Json::String(representation_value)=> Json::String(format!("{{\"x\": {}, \"y\": {}, \"z\": {}, \"representation\": {}}}", self.x, self.y, self.z, representation_value)),
			_ =>Json::String(format!("{{\"x\": {}, \"y\": {}, \"z\": {}}}", self.x, self.y, self.z))
		}
    }
}


impl Element for Avatar{
	fn get_x(&self) -> i64{
		self.get_x()
	}
	fn get_y(&self) -> i64{
		self.get_y()
	}

	fn get_z(&self) -> i64{
		self.get_z()
	}


	fn set_x(&mut self, x: i64){
		self.set_x(x);
	}

	fn set_y(&mut self, y: i64){
		self.set_y(y);
	}

	fn set_z(&mut self, z: i64){
		self.set_z(z);
	}

	fn get_representation(&self) -> &Representation{
		self.get_representation()
	}

	fn get_color(&self) -> Color{
		self.get_color()
	}

}
impl ToJson for Avatar {
    fn to_json(&self) -> Json {
		self.to_json()
    }
}
