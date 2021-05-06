#![recursion_limit = "1024"]

mod tests;
mod gen_code;

include!("../res/num_repeat.rs");
include!("../res/ident_repeat.rs");

#[macro_export]
macro_rules! create_struct_for_test {
	(@repeat (($($nums:tt),* $(,)*), $name:tt) -> ())=> {
		#[derive(Debug, Default)]
		pub struct $name {
			$( $nums: String, )*
		}
	};

	// TODO: add visibility https://danielkeep.github.io/tlborm/book/pat-visibility.html
	// TODO: implement get/set https://github.com/dtolnay/paste

	(pub struct $name:tt { $n:tt }) => {
		ident_repeat!($n, create_struct_for_test, $name)
	}
}

#[macro_export]
macro_rules! init_array {
	(@as_expr $e:expr) => {$e};

	(@repeat ((), $($args:tt)*) -> ($($body:tt)*))=> {
		crate::init_array!(@as_expr [ $($body)* ])
	};

	(@repeat (($_index:tt, $($nums:tt)*), $($args:tt)*) -> ($($body:tt)*))=> {{
		// println!("count {}", $_index);
		crate::init_array!(@repeat (($($nums)*), $($args)*) -> ($($body)* $($args)*,))
	}};

	[$e:expr; $n:tt] => {
		{
			let e = $e;
			num_repeat!($n, init_array, e.clone())
		}
	};
}

#[macro_export]
macro_rules! init_hashmap {
	(@unit $($_:tt)*) => { () };
	(@count $($key:expr),*) => (<[()]>::len(&[$(crate::init_hashmap!(@unit $key)),*]));

	($($key:expr, $value:expr),* $(,)*) => {
		{
			let _cap = crate::init_hashmap!(@count $($key),*);
			let mut _map = ::std::collections::HashMap::with_capacity(_cap);
			$(_map.insert($key, $value);)*
			_map
		}
	}
}

