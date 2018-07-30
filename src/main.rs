mod macro_;
use macro_::*;

fn macro_from_bf(code: &str) -> Macro {
	Macro::from_parts(vec![MacroPart::BfCode(code.into())])
}

fn main() {
	let value: usize = 5;
	
	let user = macro_from_bf("++--");
	let code_gen = || ".".repeat(value);
	let pre = Macro::from_code_generator(&code_gen);
	
	println!("{}", user.expand());
	println!("{}", pre.expand());
}