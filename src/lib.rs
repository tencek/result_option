#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use core::fmt::Debug;

/// A three-way enum combining `Result` and `Option`.
#[must_use = "This `ResultOption` should be handled"]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
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
            Self::None | Self::Err(_) => false,
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
            Self::None | Self::Ok(_) => false,
        }
    }

    /// Converts from `ResultOption<T, E>` to `Option<T>`, discarding the error if any.
    #[must_use]
    #[inline]
    pub fn ok(self) -> Option<T> {
        match self {
            Self::Ok(t) => Some(t),
            Self::None | Self::Err(_) => None,
        }
    }

    /// Converts from `ResultOption<T, E>` to `Option<E>`, discarding the value if any.
    #[must_use]
    #[inline]
    pub fn err(self) -> Option<E> {
        match self {
            Self::Err(e) => Some(e),
            Self::None | Self::Ok(_) => None,
        }
    }

    /// Converts from `ResultOption<T, E>` to `ResultOption<&T, &E>`, borrowing the values if they exist.
    #[inline]
    pub const fn as_ref(&self) -> ResultOption<&T, &E> {
        match self {
            Self::Ok(t) => ResultOption::Ok(t),
            Self::None => ResultOption::None,
            Self::Err(e) => ResultOption::Err(e),
        }
    }

    /// Converts from `ResultOption<T, E>` to `ResultOption<&mut T, &mut E>`, mutably borrowing the values if they exist.
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
            Self::None | Self::Err(_) => default,
        }
    }

    /// Maps an `Ok` value using the provided function, or computes a default value if `None` or `Err`.
    #[must_use]
    #[inline]
    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Self::Ok(t) => f(t),
            Self::None | Self::Err(_) => default(),
        }
    }

    /// Maps an `Ok` value using the provided function, or returns the default value of `U` if `None` or `Err`.
    #[must_use]
    #[inline]
    pub fn map_or_default<U: Default, F: FnOnce(T) -> U>(self, f: F) -> U {
        match self {
            Self::Ok(t) => f(t),
            Self::None | Self::Err(_) => U::default(),
        }
    }

    /// Maps an `Err` value using the provided function, leaving `Ok` and `None` unchanged.
    #[inline]
    pub fn map_err<F, O: FnOnce(E) -> F>(self, f: O) -> ResultOption<T, F> {
        match self {
            Self::Ok(t) => ResultOption::Ok(t),
            Self::None => ResultOption::None,
            Self::Err(e) => ResultOption::Err(f(e)),
        }
    }

    /// Maps a `None` value using the provided function, leaving `Ok` and `Err` unchanged.
    #[inline]
    pub fn inspect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Self::Ok(ref t) = self {
            f(t);
        }
        self
    }

    /// Unwraps a `ResultOption`, yielding the content of an `Ok`.
    ///
    /// # Panics
    ///
    /// Panics if the value is a `None` or `Err`, with a panic message including the
    /// passed message and the content of the `Err` (if applicable).
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// assert_eq!(x.unwrap(), 2);
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// x.unwrap(); // panics
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("emergency failure");
    /// x.unwrap(); // panics with `emergency failure`
    /// ```
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T
    where
        E: Debug,
    {
        match self {
            Self::Ok(t) => t,
            Self::None => panic!("called `ResultOption::unwrap()` on a `None` value"),
            Self::Err(e) => panic!("called `ResultOption::unwrap()` on an `Err` value: {e:?}"),
        }
    }

    /// Unwraps a `ResultOption`, yielding the content of an `Ok`.
    ///
    /// # Panics
    ///
    /// Panics if the value is a `None` or `Err`, with a panic message provided by you,
    /// and the content of the `Err` (if applicable).
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// assert_eq!(x.expect("the number should be present"), 2);
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// x.expect("testing expect with None"); // panics with `testing expect with None`
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("not found");
    /// x.expect("expected a valid result"); // panics with `expected a valid result: not found`
    /// ```
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T
    where
        E: Debug,
    {
        match self {
            Self::Ok(t) => t,
            Self::None => panic!("{msg}"),
            Self::Err(e) => panic!("{msg}: {e:?}"),
        }
    }

    /// Returns the contained `Ok` value, consuming the `self` value,
    /// without checking that the value is not `None` or `Err`.
    ///
    /// # Safety
    ///
    /// Calling this method on a `None` or `Err` value is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// assert_eq!(unsafe { x.unwrap_unchecked() }, 2);
    /// ```
    ///
    /// ```no_run
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// let y = unsafe { x.unwrap_unchecked() }; // undefined behavior!
    /// ```
    ///
    /// ```no_run
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("error");
    /// let y = unsafe { x.unwrap_unchecked() }; // undefined behavior!
    /// ```
    #[inline]
    pub unsafe fn unwrap_unchecked(self) -> T {
        debug_assert!(self.is_ok());
        match self {
            Self::Ok(t) => t,
            // SAFETY: the safety contract must be upheld by the caller.
            Self::None | Self::Err(_) => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Returns the contained `Ok` value or a provided default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`unwrap_or_else`]: ResultOption::unwrap_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let default_value = 42;
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(99);
    /// assert_eq!(x.unwrap_or(default_value), 99);
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// assert_eq!(x.unwrap_or(default_value), 42);
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("error");
    /// assert_eq!(x.unwrap_or(default_value), 42);
    /// ```
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Ok(t) => t,
            Self::None | Self::Err(_) => default,
        }
    }

    /// Returns the contained `Ok` value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let count = 21;
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(9);
    /// assert_eq!(x.unwrap_or_else(|| count * 2), 9);
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// assert_eq!(x.unwrap_or_else(|| count * 2), 42);
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("error");
    /// assert_eq!(x.unwrap_or_else(|| count * 2), 42);
    /// ```
    #[inline]
    #[track_caller]
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Self::Ok(t) => t,
            Self::None | Self::Err(_) => f(),
        }
    }

    /// Returns the contained `Ok` value or a [`default`](Default::default).
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(9);
    /// assert_eq!(x.unwrap_or_default(), 9);
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// assert_eq!(x.unwrap_or_default(), 0);
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("error");
    /// assert_eq!(x.unwrap_or_default(), 0);
    /// ```
    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Self::Ok(t) => t,
            Self::None | Self::Err(_) => T::default(),
        }
    }

    /// Unwraps a `ResultOption`, yielding the content of an `Err`.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Ok` or `None`, with a panic message including the
    /// content of the `Ok` (if applicable).
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("emergency failure");
    /// assert_eq!(x.unwrap_err(), "emergency failure");
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// x.unwrap_err(); // panics
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// x.unwrap_err(); // panics
    /// ```
    #[inline]
    #[track_caller]
    pub fn unwrap_err(self) -> E
    where
        T: Debug,
    {
        match self {
            Self::Err(e) => e,
            Self::Ok(ok) => panic!("called `ResultOption::unwrap_err()` on an `Ok` value: {ok:?}"),
            Self::None => panic!("called `ResultOption::unwrap_err()` on a `None` value"),
        }
    }

    /// Unwraps a `ResultOption`, yielding the content of an `Err`.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Ok` or `None`, with a panic message provided by you,
    /// and the content of the `Ok` (if applicable).
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("not found");
    /// assert_eq!(x.expect_err("should be an error"), "not found");
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(42);
    /// x.expect_err("testing expect_err with Ok"); // panics with custom message
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// x.expect_err("testing expect_err with None"); // panics with custom message
    /// ```
    #[inline]
    #[track_caller]
    pub fn expect_err(self, msg: &str) -> E
    where
        T: Debug,
    {
        match self {
            Self::Err(e) => e,
            Self::Ok(ok) => panic!("{msg}: {ok:?}"),
            Self::None => panic!("{msg}"),
        }
    }

    /// Returns the contained `Err` value, consuming the `self` value,
    /// without checking that the value is not `Ok` or `None`.
    ///
    /// # Safety
    ///
    /// Calling this method on an `Ok` or `None` value is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("emergency failure");
    /// assert_eq!(unsafe { x.unwrap_err_unchecked() }, "emergency failure");
    /// ```
    ///
    /// ```no_run
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// let y = unsafe { x.unwrap_err_unchecked() }; // undefined behavior!
    /// ```
    ///
    /// ```no_run
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// let y = unsafe { x.unwrap_err_unchecked() }; // undefined behavior!
    /// ```
    #[inline]
    pub unsafe fn unwrap_err_unchecked(self) -> E {
        debug_assert!(self.is_err());
        match self {
            Self::Err(e) => e,
            // SAFETY: the safety contract must be upheld by the caller.
            Self::Ok(_) | Self::None => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Unwraps a `ResultOption`, returning `Some` value if `Ok`, `None` if `None`, or panicking if `Err`.
    ///
    /// This treats the `ResultOption` as an `Option<T>`, where both `None` and `Err` are
    /// considered "no value" cases, but only `Err` causes a panic.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Err`, with a panic message including the content of the `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// assert_eq!(x.unwrap_option(), Some(2));
    ///
    /// let y: ResultOption<u32, &str> = ResultOption::None;
    /// assert_eq!(y.unwrap_option(), None);
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("emergency failure");
    /// x.unwrap_option(); // panics with `emergency failure`
    /// ```
    #[inline]
    #[track_caller]
    pub fn unwrap_option(self) -> Option<T>
    where
        E: Debug,
    {
        match self {
            Self::Ok(t) => Some(t),
            Self::None => None,
            Self::Err(e) => {
                panic!("called `ResultOption::unwrap_option()` on an `Err` value: {e:?}")
            }
        }
    }

    /// Converts to `Option<T>`, returning `Some` value if `Ok`, `None` if `None`, or panicking with a custom message if `Err`.
    ///
    /// This treats the `ResultOption` as an `Option<T>`, where both `None` and `Err` are
    /// considered "no value" cases, but only `Err` causes a panic with your custom message.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Err`, with a panic message provided by you,
    /// and the content of the `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// assert_eq!(x.expect_option("should not be an error"), Some(2));
    ///
    /// let y: ResultOption<u32, &str> = ResultOption::None;
    /// assert_eq!(y.expect_option("should not be an error"), None);
    /// ```
    ///
    /// ```should_panic
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("not found");
    /// x.expect_option("expected success or none"); // panics with custom message + error
    /// ```
    #[inline]
    #[track_caller]
    pub fn expect_option(self, msg: &str) -> Option<T>
    where
        E: Debug,
    {
        match self {
            Self::Ok(t) => Some(t),
            Self::None => None,
            Self::Err(e) => panic!("{msg}: {e:?}"),
        }
    }

    /// Returns the contained value as `Option<T>`, consuming the `self` value,
    /// without checking that the value is not `Err`.
    ///
    /// This converts `Ok(t)` to `Some(t)` and `None` to `None`, but assumes
    /// there is no `Err` variant present.
    ///
    /// # Safety
    ///
    /// Calling this method on an `Err` value is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// assert_eq!(unsafe { x.unwrap_option_unchecked() }, Some(2));
    ///
    /// let y: ResultOption<u32, &str> = ResultOption::None;
    /// assert_eq!(unsafe { y.unwrap_option_unchecked() }, None);
    /// ```
    ///
    /// ```no_run
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("error");
    /// let y = unsafe { x.unwrap_option_unchecked() }; // undefined behavior!
    /// ```
    #[inline]
    pub unsafe fn unwrap_option_unchecked(self) -> Option<T> {
        debug_assert!(!self.is_err());
        match self {
            Self::Ok(t) => Some(t),
            Self::None => None,
            // SAFETY: the safety contract must be upheld by the caller.
            Self::Err(_) => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Converts to `Option<T>`, providing a default value for `Err` cases.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// assert_eq!(x.unwrap_option_or_some(42), Some(2));
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// assert_eq!(x.unwrap_option_or_some(42), None);
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("error");
    /// assert_eq!(x.unwrap_option_or_some(42), Some(42));
    /// ```
    #[inline]
    pub fn unwrap_option_or_some(self, default: T) -> Option<T> {
        match self {
            Self::Ok(t) => Some(t),
            Self::None => None,
            Self::Err(_) => Some(default),
        }
    }

    /// Converts to `Option<T>`, using the default value of `T` for `Err` cases.
    ///
    /// This is like `unwrap_option_or_some` but uses `T::default()` instead of requiring
    /// a parameter, making it convenient when the default value is appropriate.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(42);
    /// assert_eq!(x.unwrap_option_or_some_default(), Some(42));
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// assert_eq!(x.unwrap_option_or_some_default(), None);
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("error");
    /// assert_eq!(x.unwrap_option_or_some_default(), Some(0)); // 0 is default for u32
    /// ```
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// // With String, the default is an empty string
    /// let x: ResultOption<String, i32> = ResultOption::Err(404);
    /// assert_eq!(x.unwrap_option_or_some_default(), Some(String::new()));
    ///
    /// let x: ResultOption<String, i32> = ResultOption::Ok("hello".to_string());
    /// assert_eq!(x.unwrap_option_or_some_default(), Some("hello".to_string()));
    /// ```
    #[inline]
    pub fn unwrap_option_or_some_default(self) -> Option<T>
    where
        T: Default,
    {
        match self {
            Self::Ok(t) => Some(t),
            Self::None => None,
            Self::Err(_) => Some(T::default()),
        }
    }

    /// Converts to `Option<T>` without panicking.
    ///
    /// Unlike `unwrap_option()`, this never panics - it converts `Err` to `None`
    /// just like `None` already becomes `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Ok(2);
    /// assert_eq!(x.unwrap_option_or_none(), Some(2));
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::None;
    /// assert_eq!(x.unwrap_option_or_none(), None);
    ///
    /// let x: ResultOption<u32, &str> = ResultOption::Err("error");
    /// assert_eq!(x.unwrap_option_or_none(), None); // No panic!
    /// ```
    #[inline]
    pub fn unwrap_option_or_none(self) -> Option<T> {
        match self {
            Self::Ok(t) => Some(t),
            Self::None | Self::Err(_) => None, // Both None and Err become None
        }
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
    /// Converts an `Option<T>` into a `ResultOption<T, E>` by taking ownership.
    ///
    /// This is useful when you have an owned `Option<T>` and want to convert it
    /// to a three-way enum for more structured error handling.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    ///
    /// // Real-world example: Using Vec::pop() which returns Option<T>
    /// let mut numbers = vec![42];
    ///
    /// // Convert Vec::pop() result to ResultOption
    /// let last_item: ResultOption<i32, String> = ResultOption::from(numbers.pop());
    /// assert_eq!(last_item, ResultOption::Ok(42));
    ///
    /// let no_item: ResultOption<i32, String> = ResultOption::from(numbers.pop());
    /// assert_eq!(no_item, ResultOption::None);
    ///
    /// // Using String::strip_prefix which returns Option<&str>
    /// let text = "Hello, world!";
    /// let stripped: ResultOption<&str, ()> = text.strip_prefix("Hello, ").into();
    /// assert_eq!(stripped, ResultOption::Ok("world!"));
    ///
    /// let no_match: ResultOption<&str, ()> = text.strip_prefix("Hi, ").into();
    /// assert_eq!(no_match, ResultOption::None);
    /// ```
    fn from(o: Option<T>) -> Self {
        match o {
            Some(t) => Self::Ok(t),
            None => Self::None,
        }
    }
}

impl<T: Clone, E> From<Option<&T>> for ResultOption<T, E> {
    /// Converts an `Option<&T>` into a `ResultOption<T, E>` by cloning the inner value.
    ///
    /// This is particularly useful when working with collections that return references,
    /// such as `HashMap::get()` or `BTreeMap::get()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    /// use std::collections::BTreeMap;
    ///
    /// // Real-world scenario: Looking up values in a BTreeMap
    /// let mut scores = BTreeMap::new();
    /// scores.insert("Alice", 95);
    /// scores.insert("Bob", 87);
    ///
    /// // Convert map lookups to ResultOption
    /// let alice_score: ResultOption<i32, String> = scores.get("Alice").into();
    /// assert_eq!(alice_score, ResultOption::Ok(95));
    ///
    /// let diana_score: ResultOption<i32, String> = scores.get("Diana").into();
    /// assert_eq!(diana_score, ResultOption::None);
    ///
    /// // Can also use explicit From
    /// let bob_score: ResultOption<i32, String> = ResultOption::from(scores.get("Bob"));
    /// assert_eq!(bob_score, ResultOption::Ok(87));
    /// ```
    fn from(o: Option<&T>) -> Self {
        match o {
            Some(t) => Self::Ok(t.clone()),
            None => Self::None,
        }
    }
}

/// Support for `UnwrapInfallible` trait when error type is `Infallible`.
#[cfg(feature = "unwrap_infallible")]
mod infallible;
