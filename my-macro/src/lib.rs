#![recursion_limit = "1024"]

mod tests;

include!("../res/macro_repeat.rs");

#[macro_export]
macro_rules! init_array {
	(@unit $_:tt) => { () };
	(@count $($add:tt)*) => (<[()]>::len(&[$(crate::init_array!(@unit $add)),*]));

	(@as_expr $e:expr) => {$e};

	(@repeat ((), $($args:tt)*) -> ($($c:tt)*), ($($body:tt)*))=> {{
		// let _cap = crate::init_array!(@count $($c)*);
		// println!("count {}", _cap);
		crate::init_array!(@as_expr [ $($body)* ])
	}};

	(@repeat ((+ $($count:tt)*), $($args:tt)*) -> ($($c:tt)*), ($($body:tt)*))=> {{
		// let _cap = crate::init_array!(@count $($c)*);
		// println!("count {}", _cap);
		crate::init_array!(@repeat (($($count)*), $($args)*) -> ($($c)* +), ($($body)* $($args)*,))
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

