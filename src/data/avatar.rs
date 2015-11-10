use data::world::Element;
use rustbox::Color;
use rendering::renderer_ascii::Representation;

pub struct Avatar{
	x: i32,
	y: i32,
	z: i32,
	representation: Representation
}

impl Avatar{
	pub fn _new() -> Avatar{
		Avatar{x: 0, y: 0, z: 0, representation: Representation::new('X')}
	}

	pub fn new_with_representation(representation: Representation) -> Avatar{
		Avatar{x: 0, y: 0, z: 0, representation: representation}
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
	pub fn get_x(&self) -> i32{
		self.x
	}
	pub fn get_y(&self) -> i32{
		self.y
	}

	pub fn get_z(&self) -> i32{
		self.z
	}


	pub fn set_x(&mut self, x: i32){
		self.x = x;
	}

	pub fn set_y(&mut self, y: i32){
		self.y = y;

	}

	pub fn set_z(&mut self, z: i32){
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
}


impl Element for Avatar{
	fn get_x(&self) -> i32{
		self.get_x()
	}
	fn get_y(&self) -> i32{
		self.get_y()
	}

	fn get_z(&self) -> i32{
		self.get_z()
	}


	fn set_x(&mut self, x: i32){
		self.set_x(x);
	}

	fn set_y(&mut self, y: i32){
		self.set_y(y);
	}

	fn set_z(&mut self, z: i32){
		self.set_z(z);
	}

	fn get_representation(&self) -> &Representation{
		self.get_representation()
	}

	fn get_color(&self) -> Color{
		self.get_color()
	}
}
