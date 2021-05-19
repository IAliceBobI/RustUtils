use super::*;

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
		$crate::ErrorInfo::new(Some($err.to_owned()), file!(), line!(), column!())
	}};
	($err: expr, $($args:expr),* $(,)*) => {{
		$crate::d!(format!($err, $err))
	}};
	(@$msg: expr) => {
		$crate::d!(format!("{:#?}", $msg))
	};
	() => {{
		$crate::ErrorInfo::new(None, file!(), line!(), column!())
	}};
}

#[macro_export]
macro_rules! bail {
	($msg: expr, $($args:expr),* $(,)*) => {
		Err($crate::ge!($msg, $($args)*))
	};
	($msg: expr) => {
		Err($crate::ge!($msg))
	};
}

#[macro_export]
macro_rules! ge {
	($msg: expr, $($args:expr),* $(,)*) => {
		$crate::ge!(format!($msg, $($args)*))
	};
	($msg: expr) => {{
		$crate::MyError::new($crate::d!(), Some(anyhow::anyhow!($msg)), None)
	}};
	(@$msg: expr) => {
		$crate::ge!(format!("{:#?}", $msg))
	};
}

#[macro_export]
macro_rules! datetime {
	($ts: expr) => {{
		crate::utils::gen_datetime($ts as i64)
	}};
	() => {{
		$crate::datetime!($crate::ts!())
	}};
}

/// get current UTC-timestamp
#[macro_export]
macro_rules! ts {
	() => {{
		std::time::SystemTime::now()
			.duration_since(std::time::SystemTime::UNIX_EPOCH)
			.unwrap_or_default()
			.as_secs()
	}};
}

/// generate a 'formated DateTime'
#[inline(always)]
pub fn gen_datetime(ts: i64) -> String {
	time::OffsetDateTime::from_unix_timestamp(ts).format("%F %T")
}

#[inline(always)]
pub fn get_pidns(pid: u32) -> Result<String> {
	se!(std::fs::read_link(format!("/proc/{}/ns/pid", pid)))	
		.c(crate::d!())
		.map(|p| {
			p.to_string_lossy()
				.trim_start_matches("pid:[")
				.trim_end_matches(']')
				.to_owned()
		})
}

#[macro_export]
macro_rules! se {
	($result: expr) => {
		$result.map_err(|e| anyhow::anyhow!(e))
	};
}
