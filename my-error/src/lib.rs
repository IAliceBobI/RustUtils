use lazy_static::lazy_static;
use std::fmt::{Debug, Display};
use std::sync::Mutex;
pub use anyhow;

#[macro_use]
mod utils;

// ---------- error info -------------

#[derive(Debug)]
pub struct ErrorInfo {
	pub comment: Option<String>,
	pub file: &'static str,
	pub line: u32,
	pub column: u32,
}

impl ErrorInfo {
	pub fn new(comment: Option<String>, file: &'static str, line: u32, column: u32) -> Self {
		Self { comment, file, line, column }
	}
}

impl Display for ErrorInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} At: {} Line: {} Column: {}",
			self.comment.clone().unwrap_or_default(), self.file, self.line, self.column
		)
	}
}

// ----------------- Error -------------------

#[derive(Debug)]
pub struct MyError {
	info: ErrorInfo,
	err: Option<anyhow::Error>,
	cause: Option<Box<MyError>>,
}

impl MyError {
	pub fn print_die(&self) -> ! {
		self.print();
		panic!();
	}

	pub fn print(&self) {
		eprintln!("{}", self);
	}

	fn stringify_chain(&self) -> String {
		let mut res = "\nError: ".to_owned();
		res.push_str(&self.get_top_info().to_string());
		if let Some(err) = &self.err {
			res.push_str(&err.to_string())
		}
		let mut e = self.cause();
		let mut indent_num = 0;
		while let Some(c) = e {
			let mut prefix = "\n".to_owned();
			(0..indent_num).for_each(|_| {
				prefix.push_str("    ");
			});
			res.push_str(&prefix);
			res.push_str("Caused By: ");
			res.push_str(&c.get_top_info().to_string().replace("\n", &prefix));
			if let Some(err) = &c.err {
				res.push_str(" Err: ");
				res.push_str(&err.to_string())
			}
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

		let pid = std::process::id();

		let ns = utils::get_pidns(pid).unwrap();

		let mut logn = LOG_LK.lock().unwrap();
		let mut res = generate_log_header(*logn, pid, ns);

		res.push_str(&self.stringify_chain());
		*logn += 1;
		res
	}
	fn get_top_info(&self) -> &ErrorInfo {
		&self.info
	}
	fn cause(&self) -> Option<&MyError> {
		self.cause.as_deref()
	}
	pub fn get_root_error(&self) -> &anyhow::Error {
		lazy_static! {
			static ref DEFAULT_ERROR: anyhow::Error = anyhow::anyhow!("Sorry");
		}
		let mut cause = self.cause();
		let mut err = &self.err;
		while let Some(e) = cause {
			cause = e.cause();
			err = &e.err;
		}
		err.as_ref().unwrap_or(&DEFAULT_ERROR)
	}
}

impl Display for MyError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.generate_log())
	}
}

impl MyError {
	pub fn new(info: ErrorInfo, err: Option<anyhow::Error>, cause: Option<Box<MyError>>) -> Self {
		Self { info, err, cause }
	}
}

// ----------------- Result -------------------

pub type Result<T> = std::result::Result<T, MyError>;

pub trait MyResultTrait<T> {
	fn c(self, info: ErrorInfo) -> Result<T>;
}

impl<T> MyResultTrait<T> for Result<T> {
	fn c(self, info: ErrorInfo) -> Result<T> {
		self.map_err(|e| MyError::new(info, None, Some(Box::new(e))))
	}
}

impl<T> MyResultTrait<T> for Option<T> {
	fn c(self, info: ErrorInfo) -> Result<T> {
		self.ok_or_else(|| MyError::new(info, Some(anyhow::anyhow!("None")), None))
	}
}

impl<T> MyResultTrait<T> for anyhow::Result<T> {
	fn c(self, info: ErrorInfo) -> Result<T> {
		self.map_err(|e| { MyError::new(info, Some(e), None) })
	}
}
