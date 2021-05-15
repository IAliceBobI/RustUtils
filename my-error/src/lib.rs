use lazy_static::lazy_static;
use std::fmt::{Debug, Display};
use std::sync::Mutex;

#[macro_export]
macro_rules! pnk {
	($ops: expr) => {{
		$ops.c($crate::d!()).unwrap_or_else(|e| e.print_die())
	}};
	($ops: expr, $msg: expr) => {{
		$ops.c($crate::d!($msg)).unwrap_or_else(|e| e.print_die())
	}};
}

#[macro_export]
macro_rules! d {
	($err: expr) => {{
		$crate::SimpleMsg::new($err, file!(), line!(), column!())
	}};
	(@$err: expr) => {{
		$crate::d!(format!("{:?}", $err))
	}};
	() => {{
		$crate::d!("...")
	}};
}

#[macro_export]
macro_rules! eg {
	($msg: expr) => {{
		Box::new($crate::MyError::new($crate::d!($msg), None)) as Box<dyn $crate::MyErrorTrait>
	}};
	(@$msg: expr) => {
		$crate::eg!(format!("{:#?}", $msg))
	};
	() => {
		$crate::eg!("...")
	};
}

#[macro_export]
macro_rules! datetime {
	($ts: expr) => {{
		$crate::gen_datetime($ts as i64)
	}};
	() => {{
		$crate::datetime!($crate::ts!())
	}};
}

/// get current UTC-timestamp
#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! ts {
	() => {{
		std::time::SystemTime::now()
			.duration_since(std::time::SystemTime::UNIX_EPOCH)
			.unwrap_or_default()
			.as_secs()
	}};
}

/// get current UTC-timestamp
#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! ts {
	() => {
		0
	};
}

/// generate a 'formated DateTime'
#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub fn gen_datetime(ts: i64) -> String {
	time::OffsetDateTime::from_unix_timestamp(ts).format("%F %T")
}

/// generate a 'formated DateTime'
#[cfg(target_arch = "wasm32")]
#[inline(always)]
pub fn gen_datetime(_ts: i64) -> String {
	"0000-00-00 00:00:00".to_owned()
}

#[inline(always)]
#[cfg(target_os = "linux")]
fn get_pidns(pid: u32) -> Result<String> {
	std::fs::read_link(format!("/proc/{}/ns/pid", pid))
		.c(crate::d!())
		.map(|p| {
			p.to_string_lossy()
				.trim_start_matches("pid:[")
				.trim_end_matches(']')
				.to_owned()
		})
}

#[inline(always)]
#[cfg(not(target_os = "linux"))]
#[allow(clippy::unnecessary_wraps)]
fn get_pidns(_pid: u32) -> Result<String> {
	Ok("NULL".to_owned())
}

#[derive(Debug)]
pub struct SimpleMsg<E: Debug + Display + Send + 'static> {
	pub err: E,
	pub file: String,
	pub line: u32,
	pub column: u32,
}

impl<E: Debug + Display + Send + 'static> SimpleMsg<E> {
	pub fn new(err: E, file: &str, line: u32, column: u32) -> Self {
		SimpleMsg {
			err,
			file: file.to_owned(),
			line,
			column,
		}
	}
}

impl<E: Debug + Display + Send + 'static> Display for SimpleMsg<E> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}\nAt: {} Line: {} Column: {}",
			self.err, self.file, self.line, self.column
		)
	}
}

pub trait MyErrorTrait: Display + Debug + Send {
	fn print_die(&self) -> ! {
		self.print();
		panic!();
	}

	fn print(&self) {
		eprintln!("{}", self);
	}

	fn get_top_error(&self) -> String;

	/// point to a error which caused current error
	fn cause(&self) -> Option<&dyn MyErrorTrait>; 

	fn stringify_chain(&self) -> String {
		let mut res = "\nError: ".to_owned();
		res.push_str(&self.get_top_error());
		let mut e = self.cause();
		let mut indent_num = 0;
		while let Some(c) = e {
			let mut prefix = "\n".to_owned();
			(0..indent_num).for_each(|_| {
				prefix.push_str("    ");
			});
			res.push_str(&prefix);
			res.push_str("Caused By: ");
			res.push_str(&c.get_top_error().replace("\n", &prefix));
			indent_num += 1;
			e = c.cause();
		}
		res
	}

	fn generate_log(&self) -> String {
		lazy_static! {
			// avoid out-of-order printing
			static ref LOG_LK: Mutex<u64> = Mutex::new(0);
		}

		#[inline(always)]
		fn generate_log_header(idx: u64, pid: u32, ns: String) -> String {
			format!(
				"\n# {time} [idx: {n}] [pid: {pid}] [pidns: {ns}]",
				time = crate::datetime!(),
				n = idx,
				pid = pid,
				ns = ns,
			)
		}

		#[cfg(target_arch = "wasm32")]
		let pid = 0;

		#[cfg(not(target_arch = "wasm32"))]
		let pid = std::process::id();

		let ns = get_pidns(pid).unwrap();

		let mut logn = LOG_LK.lock().unwrap();
		let mut res = generate_log_header(*logn, pid, ns);

		res.push_str(&self.stringify_chain());
		*logn += 1;
		res
	}
}

// ----------------- Error -------------------

#[derive(Debug)]
pub struct MyError<E: Debug + Display + Send + 'static> {
	msg: SimpleMsg<E>,
	cause: Option<Box<dyn MyErrorTrait>>,
}

impl<E: Debug + Display + Send + 'static> MyErrorTrait for MyError<E> {
	fn get_top_error(&self) -> String {
		self.msg.to_string()
	}
	fn cause(&self) -> Option<&dyn MyErrorTrait> {
		self.cause.as_deref()
	}
}

impl<E: Debug + Display + Send + 'static> Display for MyError<E> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.generate_log())
	}
}

impl<E: Debug + Display + Send + 'static> MyError<E> {
	pub fn new(msg: SimpleMsg<E>, cause: Option<Box<dyn MyErrorTrait>>) -> Self {
		Self { msg, cause }
	}
}

impl<E: Debug + Display + Send + 'static> From<MyError<E>> for Box<dyn MyErrorTrait> {
	fn from(e: MyError<E>) -> Self {
		Box::new(e)
	}
}

// ----------------- Result -------------------

pub type Result<T> = std::result::Result<T, Box<dyn MyErrorTrait>>;

pub trait MyResultTrait<T, E: Debug + Display + Send> {
	fn c(self, msg: SimpleMsg<E>) -> Result<T>;
}

impl<T, E: Debug + Display + Send> MyResultTrait<T, E> for Result<T> {
	fn c(self, msg: SimpleMsg<E>) -> Result<T> {
		self.map_err(|e| MyError::new(msg, Some(e)).into())
	}
}

impl<T, E: Debug + Display + Send> MyResultTrait<T, E> for Option<T> {
	fn c(self, msg: SimpleMsg<E>) -> Result<T> {
		self.ok_or_else(|| MyError::new(msg, None).into())
	}
}

impl<T, E: Debug + Display + Send, ERR: std::error::Error> MyResultTrait<T, E>
	for std::result::Result<T, ERR>
{
	fn c(self, msg: SimpleMsg<E>) -> Result<T> {
		self.map_err(|e| {
			let inner = SimpleMsg::new(e.to_string(), "_.rs", 0, 0);
			MyError::new(msg, Some(Box::new(MyError::new(inner, None)))).into()
		})
	}
}
