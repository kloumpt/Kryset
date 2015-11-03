use data::avatar::Avatar;

pub struct User<'a> {
	pub id: u32,
	name: String,
	avatar: Option<&'a mut Avatar>
}

impl<'a> User<'a>{
	pub fn new(name: &str) -> User<'a>{
		User{id: 0, name: name.to_string(), avatar: Option::None}
	}

	pub fn new_with_avatar(name: &str, avatar: &'a mut Avatar) -> User<'a>{
		User{id: 0, name: name.to_string(), avatar: Some(avatar)}
	}


	pub fn get_name(&self) -> &str{
		return &self.name;
	}
}
