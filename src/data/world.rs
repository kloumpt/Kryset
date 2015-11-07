use std::rc::Rc;
use std::cell::RefCell;
use rustbox::Color;

pub trait Element {
	fn get_x(&self) -> i32;
	fn get_y(&self) -> i32;
	fn get_z(&self) -> i32;
	fn get_representation(&self) -> char;
	fn get_color(&self) -> Color;
}

pub struct World{
	objects: Vec<Rc<RefCell<Element>>>
}

pub struct Tree {
	x: i32,
	y: i32,
	z: i32,
	representation: char
}

impl Tree {
	pub fn new(x: i32, y: i32, z: i32) -> Tree{
		Tree{x: x, y: y, z: z, representation: 'T'}
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

	fn get_color(&self) -> Color{
		Color::Green
	}
}

impl Element for Tree{
	fn get_x(&self) -> i32{
		self.get_x()
	}
	fn get_y(&self) -> i32{
		self.get_y()
	}

	fn get_z(&self) -> i32{
		self.get_z()
	}
	fn get_representation(&self) -> char{
		self.get_representation()
	}

	fn get_color(&self) -> Color{
		self.get_color()
	}
}

impl World{
	pub fn new() -> World{
		World{objects: Vec::new()}
	}

	pub fn add_object(&mut self, o: Rc<RefCell<Element>>){
		self.objects.push(o)
	}

	pub fn get_object(&self, index: usize) -> Option<Rc<RefCell<Element>>>{
		if index >= self.objects.len() {
			return None
		}
		Some(self.objects[index].clone())
	}
	pub fn get_object_amount(&self) -> usize{
		self.objects.len()
	}

}
