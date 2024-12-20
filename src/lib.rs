use std::{collections::TryReserveError, ops::RangeBounds};

/// A possible candidate trait for other modifiable things
pub trait FluentMut: Sized {
	fn f_clear(self) -> Self;
	fn f_insert(self, idx: usize, ch: char) -> Self;
	fn f_insert_str(self, idx: usize, string: &str)-> Self;
	fn f_push(self, ch: char)-> Self;
	fn f_push_str(self, string: &str)-> Self;
	fn f_replace_range<R>(self, range: R, replace_with: &str)-> Self
		where R: RangeBounds<usize>;
	fn f_reserve(self, additional: usize)-> Self;
	fn f_reserve_exact(self, additional: usize)-> Self;
	fn f_retain<F>(self, f: F)-> Self
		where F: FnMut(char) -> bool;
	fn f_shrink_to(self, min_capacity: usize) -> Self;
	fn f_shrink_to_fit(self) -> Self;
	fn f_truncate(self, new_len: usize) -> Self;
	fn f_try_reserve(self, additional: usize) -> Result<Self, TryReserveError>;
	fn f_try_reserve_exact(self, additional: usize) -> Result<Self, TryReserveError>;
}

/// Fluent versions of all `the std::string:String` mutation methods that
/// otherwise return nothing.
impl FluentMut for String {
	fn f_clear(mut self) -> Self {
		self.clear();
		self
	}

	fn f_insert(mut self, idx: usize, ch: char) -> Self {
		self.insert(idx,ch);
		self
	}

	fn f_insert_str(mut self, idx: usize, string: &str)-> Self {
		self.insert_str(idx,string);
		self
	}

	fn f_push(mut self, ch: char)-> Self {
		self.push(ch);
		self
	}

	fn f_push_str(mut self, string: &str)-> Self {
		self.push_str(string);
		self
	}

	fn f_replace_range<R>(mut self, range: R, replace_with: &str)-> Self
			where R: RangeBounds<usize> {
		self.replace_range(range,replace_with);
		self
	}

	fn f_reserve(mut self, additional: usize)-> Self {
		self.reserve(additional);
		self
	}

	fn f_reserve_exact(mut self, additional: usize)-> Self {
		self.reserve_exact(additional);
		self
	}

	fn f_retain<F>(mut self, f: F)-> Self
			where F: FnMut(char) -> bool {
		self.retain(f);
		self
	}

	fn f_shrink_to(mut self, min_capacity: usize) -> Self {
		self.shrink_to(min_capacity);
		self
	}

	fn f_shrink_to_fit(mut self) -> Self {
		self.shrink_to_fit();
		self
	}

	fn f_truncate(mut self, new_len: usize) -> Self {
		self.truncate(new_len);
		self
	}

	fn f_try_reserve(mut self, additional: usize) -> Result<Self, TryReserveError> {
		self.try_reserve(additional).map(|()| self)
	}

	fn f_try_reserve_exact(mut self, additional: usize) -> Result<Self, TryReserveError> {
		self.try_reserve_exact(additional).map(|()| self)
	}
}