
/// only for gen code
fn main() {
	let mark = "+";
	for i in 0..=1000 {
		println!("({}, $callback:ident, $($args:tt)*) => {{$callback!(@repeat (({}), $($args)*) -> (), ())}};", i, mark.repeat(i));
	}
}
