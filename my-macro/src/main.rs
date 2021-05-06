use std::fs;

/// only for gen code
fn main() {
	let mut output = String::new();
	output.push_str("#[macro_export] macro_rules! macro_repeat {");
	output.push('\n');
	for i in 0..=200usize {
		let mut nums: String = (0..i).into_iter().map(|x|x.to_string()).collect::<Vec<_>>().join(",");
		nums.push(',');
		output.push_str(&format!("({}, $callback:ident, $($args:tt)*) => {{$callback!(@repeat (({}), $($args)*) -> ())}};", i, nums));
		output.push('\n');
	}
	output.push_str("}");
	output.push('\n');
	fs::write("my-macro/res/macro_repeat.rs", output).unwrap();
}
