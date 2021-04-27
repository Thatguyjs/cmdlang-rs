pub struct Jump {
	pub index: usize,
	pub name: String
}

impl Jump {

	pub fn new(index: usize, name: String) -> Jump {
		Jump { index, name }
	}

}


pub struct JumpAlias {
	pub name: String,
	pub alias: usize
}

impl JumpAlias {

	pub fn new(name: String, alias: usize) -> JumpAlias {
		JumpAlias { name, alias }
	}

}
