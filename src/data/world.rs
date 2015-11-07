use data::avatar::Avatar;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Element {
	fn get_x(&self) -> i32;
	fn get_y(&self) -> i32;
	fn get_z(&self) -> i32;
	fn get_representation(&self) -> char;
	fn set_representation(mut self, representation: char);
}

pub struct World{
	objects: Vec<Rc<RefCell<Avatar>>>
}


impl World{
	pub fn new() -> World{
		World{objects: Vec::new()}
	}

	pub fn add_object(&mut self, o: Rc<RefCell<Avatar>>){
		self.objects.push(o)
	}

	pub fn get_object(&self, index: usize) -> Option<Rc<RefCell<Avatar>>>{
		if index >= self.objects.len() {
			return None
		}
		Some(self.objects[index].clone())
	}
	pub fn get_object_amount(&self) -> usize{
		self.objects.len()
	}

}
