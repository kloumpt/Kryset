use data::avatar::Avatar;
use std::rc::Rc;
use std::cell::RefCell;

use rustc_serialize::json::{ToJson, Json};

pub struct User {
	pub id: u64,
	name: String,
	avatar: Option<Rc<RefCell<Avatar>>>
}

impl User {
	pub fn new(name: &str) -> User{
		User{id: 0, name: name.to_string(), avatar: Option::None}
	}

	pub fn _new_with_avatar(name: & str, avatar: Rc<RefCell<Avatar>>) -> User{
		User{id: 0, name: name.to_string(), avatar: Option::Some(avatar)}
	}

	pub fn from_json(data: &Json)->User{
		let mut id =0u64;
		let mut name="".to_string();
		let mut avatar: Option<Rc<RefCell<Avatar>>>=None;

	    match data.as_object(){
			Some(obj) =>{
		        match obj.get("id") {
		            Some(&Json::U64(v)) => {id=v; ()},
					_=>()
				};
		        match obj.get("name") {
		            Some(v) => match v.clone(){
						Json::String(v)=>{name=v; ()},
						_ => ()
					},
					_=>()
				};

		        match obj.get("avatar") {
		            Some(v) => {avatar=Some(Rc::new(RefCell::new(Avatar::from_json(v)))); ()},
					_=>()
				};
			},
			None =>()
		};



		User{id: id, name: name, avatar: avatar}
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


    pub fn to_json(&self) -> Json {
		match self.avatar.clone(){
			Some(avatar) => match avatar.borrow().to_json(){
				Json::String(avatar_value)=>Json::String(format!("{{\"id\": {}, \"name\": \"{}\", \"avatar\": {}}}", self.id, self.name, avatar_value)),
				_ => Json::String(format!("{{\"id\": {}, \"name\": \"{}\"}}", self.id, self.name))
			},
			_ => Json::String(format!("{{\"id\": {}, \"name\": \"{}\"}}", self.id, self.name))
		}
    }
}

impl ToJson for User {
    fn to_json(&self) -> Json {
		self.to_json()
	}
}
