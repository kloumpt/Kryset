use data::avatar::Avatar;

pub struct User {
	pub id: u32,
	name: String,
	avatar_id: Option<usize>
}

impl User{
	pub fn new(name: &str) -> User{
		User{id: 0, name: name.to_string(), avatar_id: Option::None}
	}

	pub fn new_with_avatar(name: &str, avatar: usize) -> User{
		User{id: 0, name: name.to_string(), avatar_id: Option::Some(avatar)}
	}


	pub fn get_name(&self) -> &str{
		&self.name
	}

	pub fn set_avatar(&mut self, id: usize){
		self.avatar_id=Some(id);
	}

	pub fn get_avatar(&mut self) -> &mut Option<usize>{
		&mut self.avatar_id
	}
}
