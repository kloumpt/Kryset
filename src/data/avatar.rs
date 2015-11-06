use data::world::Element;
pub struct Avatar{
	x: i32,
	y: i32,
	z: i32,
	representation: char
}

impl Avatar{
	pub fn new() -> Avatar{
		Avatar{x: 0, y: 0, z: 0, representation: ' '}
	}
	pub fn new_with_representation(representation: char) -> Avatar{
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
		pub fn get_representation(&self) -> char{
			self.representation
		}

		pub fn set_representation(&mut self, representation: char){
			self.representation=representation
		}
}
