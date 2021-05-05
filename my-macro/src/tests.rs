#![cfg(test)]

use super::*;

#[test]
fn init_array() {
	let strings = init_array![String::from("hi!"); 20];
	assert_eq!(strings.len(), 20);
}

#[test]
fn init_hashmap() {
	let m = init_hashmap!(
		"a", 3,
		"b", 4,
		"c", 5,
	);
	assert_eq!(m.len(), 3);
}
