use std::{collections::TryReserveError, ops::RangeBounds};

/// A trait that provides fluent versions of the `String` mutation
/// methods, allowing for a more natural construction of strings.
///
/// # Examples
/// ```
/// use fluent_string::*;
///
/// assert_eq!("my string".to_owned()
///     .f_push_str(" is a bit longer now")
///     .f_insert_str(12,", maybe,")
///     .f_truncate(33),
///     "my string is, maybe, a bit longer");
/// ```
pub trait FluentMut: Sized {
    /// As `String::clear` except returns `self`
    #[must_use]
    fn f_clear(self) -> Self;
    /// As `String::insert` except returns `self`
    #[must_use]
    fn f_insert(self, idx: usize, ch: char) -> Self;
    /// As `String::insert_str` except returns `self`
    #[must_use]
    fn f_insert_str(self, idx: usize, string: &str) -> Self;
    /// As `String::push` except returns `self`
    #[must_use]
    fn f_push(self, ch: char) -> Self;
    /// As `String::push_str` except returns `self`
    #[must_use]
    fn f_push_str(self, string: &str) -> Self;
    /// As `String::replace_range` except returns `self`
    #[must_use]
    fn f_replace_range<R>(self, range: R, replace_with: &str) -> Self
    where
        R: RangeBounds<usize>;
    /// As `String::reserve` except returns `self`
    #[must_use]
    fn f_reserve(self, additional: usize) -> Self;
    /// As `String::reserve_exact` except returns `self`
    #[must_use]
    fn f_reserve_exact(self, additional: usize) -> Self;
    /// As `String::retain` except returns `self`
    #[must_use]
    fn f_retain<F>(self, f: F) -> Self
    where
        F: FnMut(char) -> bool;
    /// As `String::shrink_to` except returns `self`
    #[must_use]
    fn f_shrink_to(self, min_capacity: usize) -> Self;
    /// As `String::shrink_to_fit` except returns `self`
    #[must_use]
    fn f_shrink_to_fit(self) -> Self;
    /// As `String::truncate` except returns `self`
    #[must_use]
    fn f_truncate(self, new_len: usize) -> Self;
    /// As `String::try_reserve` except returns `Result<Self, TryReserveError>`
    /// # Errors
    /// See `String::try_reserve_exact`
    fn f_try_reserve(self, additional: usize) -> Result<Self, TryReserveError>;
    /// As `String::try_reserve_exact` except returns `Result<Self, TryReserveError>`
    /// # Errors
    /// See `String::try_reserve_exact`
    fn f_try_reserve_exact(self, additional: usize) -> Result<Self, TryReserveError>;
}

/// Fluent versions of all `std::string:String` mutation methods that
/// otherwise return nothing.
impl FluentMut for String {
    fn f_clear(mut self) -> Self {
        self.clear();
        self
    }

    fn f_insert(mut self, idx: usize, ch: char) -> Self {
        self.insert(idx, ch);
        self
    }

    fn f_insert_str(mut self, idx: usize, string: &str) -> Self {
        self.insert_str(idx, string);
        self
    }

    fn f_push(mut self, ch: char) -> Self {
        self.push(ch);
        self
    }

    fn f_push_str(mut self, string: &str) -> Self {
        self.push_str(string);
        self
    }

    fn f_replace_range<R>(mut self, range: R, replace_with: &str) -> Self
    where
        R: RangeBounds<usize>,
    {
        self.replace_range(range, replace_with);
        self
    }

    fn f_reserve(mut self, additional: usize) -> Self {
        self.reserve(additional);
        self
    }

    fn f_reserve_exact(mut self, additional: usize) -> Self {
        self.reserve_exact(additional);
        self
    }

    fn f_retain<F>(mut self, f: F) -> Self
    where
        F: FnMut(char) -> bool,
    {
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

/// Fluent versions of all `&mut std::string:String` mutation methods that
/// otherwise return nothing.
impl FluentMut for &mut String {
    fn f_clear(self) -> Self {
        self.clear();
        self
    }

    fn f_insert(self, idx: usize, ch: char) -> Self {
        self.insert(idx, ch);
        self
    }

    fn f_insert_str(self, idx: usize, string: &str) -> Self {
        self.insert_str(idx, string);
        self
    }

    fn f_push(self, ch: char) -> Self {
        self.push(ch);
        self
    }

    fn f_push_str(self, string: &str) -> Self {
        self.push_str(string);
        self
    }

    fn f_replace_range<R>(self, range: R, replace_with: &str) -> Self
    where
        R: RangeBounds<usize>,
    {
        self.replace_range(range, replace_with);
        self
    }

    fn f_reserve(self, additional: usize) -> Self {
        self.reserve(additional);
        self
    }

    fn f_reserve_exact(self, additional: usize) -> Self {
        self.reserve_exact(additional);
        self
    }

    fn f_retain<F>(self, f: F) -> Self
    where
        F: FnMut(char) -> bool,
    {
        self.retain(f);
        self
    }

    fn f_shrink_to(self, min_capacity: usize) -> Self {
        self.shrink_to(min_capacity);
        self
    }

    fn f_shrink_to_fit(self) -> Self {
        self.shrink_to_fit();
        self
    }

    fn f_truncate(self, new_len: usize) -> Self {
        self.truncate(new_len);
        self
    }

    fn f_try_reserve(self, additional: usize) -> Result<Self, TryReserveError> {
        self.try_reserve(additional).map(|()| self)
    }

    fn f_try_reserve_exact(self, additional: usize) -> Result<Self, TryReserveError> {
        self.try_reserve_exact(additional).map(|()| self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Owned String tests
    #[test]
    fn test_owned_clear() {
        assert_eq!("this is a string".to_string().f_clear().len(), 0);
    }
    #[test]
    fn test_owned_insert() {
        assert!("this is a string"
            .to_string()
            .f_insert(5, 'b')
            .eq_ignore_ascii_case("THIS BIS A STRING"));
    }
    #[test]
    fn test_owned_insert_str() {
        assert!("this is a string"
            .to_string()
            .f_insert_str(8, "not ")
            .eq_ignore_ascii_case("THIS IS NOT A STRING"));
    }
    #[test]
    fn test_owned_push() {
        assert!("this is a string"
            .to_string()
            .f_push('P')
            .eq_ignore_ascii_case("THIS IS A STRINGP"));
    }
    #[test]
    fn test_owned_push_str() {
        assert!("this is a string"
            .to_string()
            .f_push_str("PUP")
            .eq_ignore_ascii_case("THIS IS A STRINGPUP"));
    }
    #[test]
    fn test_owned_replace_range() {
        assert!("this is a string"
            .to_string()
            .f_replace_range(7..9, " not your")
            .eq_ignore_ascii_case("THIS IS NOT YOUR STRING"));
    }
    #[test]
    fn test_owned_reserve() {
        assert_eq!(String::with_capacity(10).f_reserve(20).capacity(), 20);
    }
    #[test]
    fn test_owned_reserve_exact() {
        assert_eq!(String::with_capacity(10).f_reserve_exact(20).capacity(), 20);
    }
    #[test]
    fn test_owned_retain() {
        assert!("this is a string"
            .to_string()
            .f_retain(|c| c != 't')
            .eq_ignore_ascii_case("HIS IS A SRING"));
    }
    #[test]
    fn test_owned_shrink_to() {
        assert_eq!(
            String::with_capacity(30)
                .f_push_str("this is a string")
                .f_shrink_to(20)
                .capacity(),
            20
        );
    }
    #[test]
    fn test_owned_shrink_to_fit() {
        assert_eq!(
            String::with_capacity(30)
                .f_push_str("this is a string")
                .f_shrink_to_fit()
                .capacity(),
            16
        );
    }
    #[test]
    fn test_owned_truncate() {
        assert!("this is a string"
            .to_string()
            .f_truncate(4)
            .eq_ignore_ascii_case("THIS"));
    }
    #[test]
    fn test_owned_try_reserve() {
        assert_eq!(
            String::with_capacity(10)
                .f_try_reserve(20)
                .unwrap()
                .capacity(),
            20
        );
    }
    #[test]
    fn test_owned_try_reserve_exact() {
        assert_eq!(
            String::with_capacity(10)
                .f_try_reserve_exact(20)
                .unwrap()
                .capacity(),
            20
        );
    }

    // String ref tests
    #[test]
    fn test_ref_clear() {
        let mut s = "this is a string".to_string();
        let s = &mut s;
        assert_eq!(s.f_clear().len(), 0);
    }
    #[test]
    fn test_ref_insert() {
        let mut s = "this is a string".to_string();
        let s = &mut s;
        assert!(s.f_insert(5, 'b').eq_ignore_ascii_case("THIS BIS A STRING"));
    }
    #[test]
    fn test_ref_insert_str() {
        let mut s = "this is a string".to_string();
        let s = &mut s;
        assert!(s
            .f_insert_str(8, "not ")
            .eq_ignore_ascii_case("THIS IS NOT A STRING"));
    }
    #[test]
    fn test_ref_push() {
        let mut s = "this is a string".to_string();
        let s = &mut s;
        assert!(s.f_push('P').eq_ignore_ascii_case("THIS IS A STRINGP"));
    }
    #[test]
    fn test_ref_push_str() {
        let mut s = "this is a string".to_string();
        let s = &mut s;
        assert!(s
            .f_push_str("PUP")
            .eq_ignore_ascii_case("THIS IS A STRINGPUP"));
    }
    #[test]
    fn test_ref_replace_range() {
        let mut s = "this is a string".to_string();
        let s = &mut s;
        assert!(s
            .f_replace_range(7..9, " not your")
            .eq_ignore_ascii_case("THIS IS NOT YOUR STRING"));
    }
    #[test]
    fn test_ref_reserve() {
        let mut s = String::with_capacity(10);
        let s = &mut s;
        assert_eq!(s.f_reserve(20).capacity(), 20);
    }
    #[test]
    fn test_ref_reserve_exact() {
        let mut s = String::with_capacity(10);
        let s = &mut s;
        assert_eq!(s.f_reserve_exact(20).capacity(), 20);
    }
    #[test]
    fn test_ref_retain() {
        let mut s = "this is a string".to_string();
        let s = &mut s;
        assert!(s
            .f_retain(|c| c != 't')
            .eq_ignore_ascii_case("HIS IS A SRING"));
    }
    #[test]
    fn test_ref_shrink_to() {
        let mut s = String::with_capacity(30);
        let s = &mut s;
        assert_eq!(
            s.f_push_str("this is a string").f_shrink_to(20).capacity(),
            20
        );
    }
    #[test]
    fn test_ref_shrink_to_fit() {
        let mut s = String::with_capacity(30);
        let s = &mut s;
        assert_eq!(
            s.f_push_str("this is a string")
                .f_shrink_to_fit()
                .capacity(),
            16
        );
    }
    #[test]
    fn test_ref_truncate() {
        let mut s = "this is a string".to_string();
        let s = &mut s;
        assert!(s.f_truncate(4).eq_ignore_ascii_case("THIS"));
    }
    #[test]
    fn test_ref_try_reserve() {
        let mut s = String::with_capacity(10);
        let s = &mut s;
        assert_eq!(s.f_try_reserve(20).unwrap().capacity(), 20);
    }
    #[test]
    fn test_ref_try_reserve_exact() {
        let mut s = String::with_capacity(10);
        let s = &mut s;
        assert_eq!(s.f_try_reserve_exact(20).unwrap().capacity(), 20);
    }
}
