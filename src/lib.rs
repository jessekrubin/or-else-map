#![doc = include_str!("../README.md")]

/// `Option::map_or_else` but with the english words in "WYSIWYG" order
pub trait OrElseMapOption<T> {
    fn or_else_map<U, D, F>(self, default: D, map: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U;
}

impl<T> OrElseMapOption<T> for Option<T> {
    #[inline]
    fn or_else_map<U, D, F>(self, default: D, map: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        Self::map_or_else(self, default, map)
    }
}

/// `Result::map_or_else` but with the english words in "WYSIWYG" order
pub trait OrElseMapResult<T, E> {
    fn or_else_map<U, D, F>(self, default: D, map: F) -> U
    where
        D: FnOnce(E) -> U,
        F: FnOnce(T) -> U;
}

impl<T, E> OrElseMapResult<T, E> for Result<T, E> {
    #[inline]
    fn or_else_map<U, D, F>(self, default: D, map: F) -> U
    where
        D: FnOnce(E) -> U,
        F: FnOnce(T) -> U,
    {
        Self::map_or_else(self, default, map)
    }
}

pub mod prelude {
    pub use super::{OrElseMapOption, OrElseMapResult};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_or_else_map() {
        let result: Result<i32, &str> = Ok(10);
        let value = result.or_else_map(|_| 0, |x| x * 2);
        assert_eq!(value, 20);

        let result: Result<i32, &str> = Err("error");
        let value = result.or_else_map(|_| 0, |x| x * 2);
        assert_eq!(value, 0);
    }

    #[test]
    fn option_or_else_map() {
        let option: Option<i32> = Some(10);
        let value = option.or_else_map(|| 0, |x| x * 2);
        assert_eq!(value, 20);

        let option: Option<i32> = None;
        let value = option.or_else_map(|| 0, |x| x * 2);
        assert_eq!(value, 0);
    }
}
