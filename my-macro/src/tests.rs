#![cfg(test)]

use super::*;

#[test]
fn init_array() {
	let strings = init_array![String::from("hi!"); 200];
	assert_eq!(strings.len(), 200);
}
