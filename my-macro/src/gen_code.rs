#![cfg(test)]

use std::fs;

#[test]
fn gen_ident_repeat() {
	let mut output = String::new();
	output.push_str("#[macro_export] macro_rules! ident_repeat {");
	output.push('\n');
	for i in 0..=200usize {
		let mut nums: String = (0..i).into_iter().map(|x|format!("i{}", x)).collect::<Vec<_>>().join(",");
		nums.push(',');
		output.push_str(&format!("({}, $callback:ident, $($args:tt)*) => {{$callback!(@repeat (({}), $($args)*) -> ())}};", i, nums));
		output.push('\n');
	}
	output.push_str("}");
	output.push('\n');
	fs::write("res/ident_repeat.rs", output).unwrap();
}

#[test]
fn gen_num_repeat() {
	// println!("{:?}", std::env::current_dir().unwrap());
	let mut output = String::new();
	output.push_str("#[macro_export] macro_rules! num_repeat {");
	output.push('\n');
	for i in 0..=200usize {
		let mut nums: String = (0..i).into_iter().map(|x|x.to_string()).collect::<Vec<_>>().join(",");
		nums.push(',');
		output.push_str(&format!("({}, $callback:ident, $($args:tt)*) => {{$callback!(@repeat (({}), $($args)*) -> ())}};", i, nums));
		output.push('\n');
	}
	output.push_str("}");
	output.push('\n');
	fs::write("res/num_repeat.rs", output).unwrap();
}
