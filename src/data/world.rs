use std::rc::Rc;
use std::cell::RefCell;
use rustbox::Color;

use rendering::renderer_ascii::Representation;

pub trait Element {
	fn get_x(&self) -> i32;
	fn get_y(&self) -> i32;
	fn get_z(&self) -> i32;

	fn set_x(&mut self, x: i32);
	fn set_y(&mut self, x: i32);
	fn set_z(&mut self, x: i32);

	fn get_representation(&self) -> &Representation;
	fn get_color(&self) -> Color;
}

pub struct World{
	objects: Vec<Rc<RefCell<Element>>>
}

pub struct Tree {
	x: i32,
	y: i32,
	z: i32,
	representation: Representation
}

impl Tree {
	pub fn new(x: i32, y: i32, z: i32) -> Tree{
		//Tree{x: x, y: y, z: z, representation: Representation::new('T')}
		Tree{x: x, y: y, z: z, representation: Representation::new_composed(vec![(2, 0, ','), (1, 1, '/'), (2, 1, 'X'), (3, 1, '\\'), (0, 2, '/'), (1, 2, 'X'), (2, 2, 'X'), (3, 2, 'X'), (4, 2, '\\'),(2, 3, 'I')])}
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
