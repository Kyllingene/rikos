#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({

	});
}

#[macro_export]
macro_rules! println
{
	() => ({
		$crate::print!("\r\n")
	});
	($fmt:expr) => ({
		$crate::print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		$crate::print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

