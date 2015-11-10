use data::avatar::Avatar;
use std::rc::Rc;
use std::cell::RefCell;

pub struct User {
	pub id: u32,
	name: String,
	avatar: Option<Rc<RefCell<Avatar>>>
}

impl User {
	pub fn new(name: &str) -> User{
		User{id: 0, name: name.to_string(), avatar: Option::None}
	}

	pub fn _new_with_avatar(name: & str, avatar: Rc<RefCell<Avatar>>) -> User<>{
		User{id: 0, name: name.to_string(), avatar: Option::Some(avatar)}
	}


	pub fn get_name(&self) -> & str{
		&self.name
	}

	pub fn set_name(&mut self, name: String){
		self.name=name;
	}

	pub fn set_avatar(&mut self, avatar: Rc<RefCell<Avatar>>){
		self.avatar=Some(avatar);
	}

	pub fn get_avatar(&self) -> Option<Rc<RefCell<Avatar>>>{

		match self.avatar.clone() {
			Some(avatar) => Some(avatar),
			None => None
		}
	}
}
