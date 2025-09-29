#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// A three-way enum combining `Result` and `Option`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultOption<T, E> {
    /// Success with value
    Ok(T),
    /// Success with no value
    None,
    /// Failure
    Err(E),
}

impl<T, E> ResultOption<T, E> {
    /// Returns `true` if the result is `Ok`.
    #[must_use]
    #[inline]
    pub const fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    /// Returns `true` if the result is `Ok` and the value inside it matches a predicate.
    #[must_use]
    #[inline]
    pub fn is_ok_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Self::Ok(t) => f(t),
            _ => false,
        }
    }

    /// Returns `true` if the result is `None`.
    #[must_use]
    #[inline]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if the result is `Err`.
    #[must_use]
    #[inline]
    pub const fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    /// Returns `true` if the result is `Err` and the error inside it matches a predicate.
    #[must_use]
    #[inline]
    pub fn is_err_and(self, f: impl FnOnce(E) -> bool) -> bool {
        match self {
            Self::Err(e) => f(e),
            _ => false,
        }
    }

    /// Converts from `ResultOption<T, E>` to `Option<T>`, discarding the error if any.
    #[must_use]
    #[inline]
    pub fn ok(self) -> Option<T> {
        match self {
            Self::Ok(t) => Some(t),
            _ => None,
        }
    }

    /// Converts from `ResultOption<T, E>` to `Option<E>`, discarding the value if any.
    #[must_use]
    #[inline]
    pub fn err(self) -> Option<E> {
        match self {
            Self::Err(e) => Some(e),
            _ => None,
        }
    }

    /// Converts from `ResultOption<T, E>` to `ResultOption<&T, &E>`, borrowing the values if they exist.
    #[must_use]
    #[inline]
    pub const fn as_ref(&self) -> ResultOption<&T, &E> {
        match self {
            Self::Ok(t) => ResultOption::Ok(t),
            Self::None => ResultOption::None,
            Self::Err(e) => ResultOption::Err(e),
        }
    }

    /// Converts from `ResultOption<T, E>` to `ResultOption<&mut T, &mut E>`, mutably borrowing the values if they exist.
    #[must_use]
    #[inline]
    pub const fn as_mut(&mut self) -> ResultOption<&mut T, &mut E> {
        match self {
            Self::Ok(t) => ResultOption::Ok(t),
            Self::None => ResultOption::None,
            Self::Err(e) => ResultOption::Err(e),
        }
    }

    /// Maps an `Ok` value using the provided function, leaving `None` and `Err` unchanged.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ResultOption<U, E> {
        match self {
            Self::Ok(t) => ResultOption::Ok(f(t)),
            Self::None => ResultOption::None,
            Self::Err(e) => ResultOption::Err(e),
        }
    }

    /// Maps an `Ok` value using the provided function, or returns a default value if `None` or `Err`.
    #[must_use]
    #[inline]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Self::Ok(t) => f(t),
            _ => default,
        }
    }

    /// Maps an `Ok` value using the provided function, or computes a default value if `None` or `Err`.
    #[must_use]
    #[inline]
    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Self::Ok(t) => f(t),
            _ => default(),
        }
    }

    /// Maps an `Ok` value using the provided function, or returns the default value of `U` if `None` or `Err`.
    #[must_use]
    #[inline]
    pub fn map_or_default<U: Default, F: FnOnce(T) -> U>(self, f: F) -> U {
        match self {
            Self::Ok(t) => f(t),
            _ => U::default(),
        }
    }

    /// Maps an `Err` value using the provided function, leaving `Ok` and `None` unchanged.
    #[must_use]
    #[inline]
    pub fn map_err<F, O: FnOnce(E) -> F>(self, f: O) -> ResultOption<T, F> {
        match self {
            Self::Ok(t) => ResultOption::Ok(t),
            Self::None => ResultOption::None,
            Self::Err(e) => ResultOption::Err(f(e)),
        }
    }

    /// Maps a `None` value using the provided function, leaving `Ok` and `Err` unchanged.
    #[must_use]
    #[inline]
    pub fn inspect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Self::Ok(ref t) = self {
            f(t);
        }
        self
    }
}

impl<T, E> From<Result<Option<T>, E>> for ResultOption<T, E> {
    fn from(r: Result<Option<T>, E>) -> Self {
        match r {
            Ok(Some(t)) => Self::Ok(t),
            Ok(None) => Self::None,
            Err(e) => Self::Err(e),
        }
    }
}

impl<T, E> From<Option<T>> for ResultOption<T, E> {
    fn from(o: Option<T>) -> Self {
        match o {
            Some(t) => Self::Ok(t),
            None => Self::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_option_some() {
        let opt: Option<i32> = Some(42);
        let result_opt: ResultOption<i32, String> = ResultOption::from(opt);
        assert_eq!(result_opt, ResultOption::Ok(42));
    }

    #[test]
    fn test_from_option_none() {
        let opt: Option<i32> = None;
        let result_opt: ResultOption<i32, String> = ResultOption::from(opt);
        assert_eq!(result_opt, ResultOption::None);
    }

    #[test]
    fn test_from_option_conversion_syntax() {
        // Test using Into trait (automatic conversion)
        let result_opt: ResultOption<String, ()> = Some("hello".to_string()).into();
        assert_eq!(result_opt, ResultOption::Ok("hello".to_string()));

        let result_opt: ResultOption<String, ()> = None.into();
        assert_eq!(result_opt, ResultOption::None);
    }
}
