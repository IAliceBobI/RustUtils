# Macro

## Dependency

my-macro = { git = "https://github.com/chenwei767/RustUtils.git", branch = "master" }

## Usage

```rust
use my_macro::*;

#[test]
fn create_struct_for_test() {
  create_struct_for_test! {pub struct AAA { String, 22 } }
  let mut obj = AAA::default();
  obj.i0 = String::from("bbbb");
  obj.i10 = String::from("bbbb");
  obj.i21 = String::from("bbbb");
  create_struct_for_test! {pub struct BBB { i32, 22 } }
  let mut obj = BBB::default();
  obj.i0 = 1;
  obj.i10 = 2;
  obj.i21 = 3;
}

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

```
