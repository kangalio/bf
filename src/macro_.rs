pub enum Macro<'a> {
	UserMacro { parts: Vec<MacroPart<'a>> },
	PredefinedMacro { code_generator: &'a Fn() -> String }
}

impl<'a> Macro<'a> {
	pub fn from_parts(parts: Vec<MacroPart<'a>>) -> Self {
		Macro::UserMacro { parts }
	}
	
	pub fn from_code_generator<F>(code_generator: &'a F) -> Self
			where F: Fn() -> String {
		Macro::PredefinedMacro { code_generator }
	}
	
	pub fn expand(&self) -> String {
		match self {
			Macro::UserMacro { parts } => Macro::expand_parts(parts),
			Macro::PredefinedMacro { code_generator } => (code_generator)()
		}
	}
	
	fn expand_parts(parts: &[MacroPart<'a>]) -> String {
		parts.iter()
			.map(|p| p.expand())
			.collect::<Vec<String>>()
			.concat()
	}
}

pub enum MacroPart<'a> {
	BfCode(&'a str),
	Macro {
		macro_: &'a Macro<'a>,
	},
}

impl<'a> MacroPart<'a> {
	pub fn expand(&self) -> String {
		match self {
			MacroPart::BfCode(code) => String::from(*code),
			MacroPart::Macro { macro_ } => macro_.expand()
		}
	}
}