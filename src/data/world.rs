use data::avatar::Avatar;

pub trait Element {
	fn get_x(&self) -> i32;
	fn get_y(&self) -> i32;
	fn get_z(&self) -> i32;
	fn get_representation(&self) -> char;
	fn set_representation(&mut self, representation: char);
}

pub struct World{
	objects: Vec<Avatar>
}


impl World{
	pub fn new() -> World{
		World{objects: Vec::new()}
	}

	pub fn add_object(&mut self, o: Avatar) -> usize{
		self.objects.push(o);
		return self.objects.len()-1
	}

	pub fn get_object(&mut self, index: usize) -> Option<&mut Avatar>{
		if index >= self.objects.len() {
			return None
		}
		Some(&mut self.objects[index])
	}
	pub fn get_object_amount(&self) -> usize{
		self.objects.len()
	}

}
