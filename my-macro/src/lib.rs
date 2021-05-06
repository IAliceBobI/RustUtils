#![recursion_limit = "1024"]

use paste::paste;

mod tests;

include!("../res/macro_repeat.rs");

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
			macro_repeat!($n, init_array, e.clone())
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

