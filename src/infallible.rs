use crate::ResultOption;
use std::convert::Infallible;
use unwrap_infallible::UnwrapInfallible;

impl<T> UnwrapInfallible for ResultOption<T, Infallible> {
    type Ok = Option<T>;

    /// Unwraps a `ResultOption<T, Infallible>` to `Option<T>`.
    ///
    /// Since the error type is `Infallible`, it's impossible for this `ResultOption`
    /// to contain an `Err` value. This method safely converts the three-way enum
    /// to a two-way `Option<T>` without any possibility of panicking.
    ///
    /// This is particularly useful when working with APIs that might return errors
    /// in general, but in specific contexts (like with `Infallible`), you know
    /// no error can occur.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_option::ResultOption;
    /// use std::convert::Infallible;
    /// use unwrap_infallible::UnwrapInfallible;
    ///
    /// // Create a ResultOption that can never fail
    /// let success: ResultOption<i32, Infallible> = ResultOption::Ok(42);
    /// assert_eq!(success.unwrap_infallible(), Some(42));
    ///
    /// let none_value: ResultOption<i32, Infallible> = ResultOption::None;
    /// assert_eq!(none_value.unwrap_infallible(), None);
    ///
    /// // The Err case is impossible with Infallible, so this would never compile:
    /// // let error: ResultOption<i32, Infallible> = ResultOption::Err(?); // No way to create Infallible
    /// ```
    ///
    /// ```
    /// use result_option::ResultOption;
    /// use std::convert::Infallible;
    /// use unwrap_infallible::UnwrapInfallible;
    ///
    /// // Real-world example: converting from a function that uses Infallible
    /// fn always_succeeds(value: Option<String>) -> ResultOption<String, Infallible> {
    ///     match value {
    ///         Some(s) => ResultOption::Ok(s),
    ///         None => ResultOption::None,
    ///         // ResultOption::Err is impossible to create with Infallible
    ///     }
    /// }
    ///
    /// let result = always_succeeds(Some("hello".to_string()));
    /// let option: Option<String> = result.unwrap_infallible();
    /// assert_eq!(option, Some("hello".to_string()));
    ///
    /// let result2 = always_succeeds(None);
    /// let option2: Option<String> = result2.unwrap_infallible();
    /// assert_eq!(option2, None);
    /// ```
    fn unwrap_infallible(self) -> Option<T> {
        match self {
            Self::Ok(t) => Some(t),
            Self::None => None,
            Self::Err(never) => match never {}, // Infallible can never occur
        }
    }
}
