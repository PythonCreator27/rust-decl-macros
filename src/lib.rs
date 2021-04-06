/// A version of the standard library's `vec![]` macro.
/// ## How to use
/// ### Using repetition
/// ```
/// let _: Vec<u32> = avec::avec![42; 4];
/// ```
/// ### Using the regular way
/// ```
/// let _: Vec<u32> = avec::avec![42, 4];
/// ```
/// ### Trailing commas
/// ```
/// let _: Vec<u32> = avec::avec![42, 4,];
/// ```
/// ## How not to use
/// ```compile_fail
/// let _: Vec<u32> = avec::avec![42; "foo"];
/// ```
/// ```compile_fail
/// let _: Vec<u32> = avec::avec![,];
/// ```
#[macro_export]
macro_rules! avec {
    ($($e:expr),*) => {{
        const C: usize = $crate::count![@COUNT; $($e),*];
        #[allow(unused_mut)]
        let mut vec = ::std::vec::Vec::with_capacity(C);
        $(vec.push($e);)*
        vec
    }};
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    ($e:expr; $count:expr) => {{
        let mut vec = ::std::vec::Vec::new();
        vec.resize($count, $e);
        vec
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($e:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $e]),*])
    };
    (@SUBST; $_e:expr) => { () };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let vec: Vec<i32> = avec![];
        assert!(vec.is_empty());
    }

    #[test]
    fn one_item() {
        let vec: Vec<i32> = avec![42];
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 42);
    }

    #[test]
    fn two_items() {
        let vec: Vec<i32> = avec![42, 2];
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 42);
        assert_eq!(vec[1], 2);
    }

    #[test]
    fn trailing_comma() {
        let vec: Vec<i32> = avec![42, 2,];
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 42);
        assert_eq!(vec[1], 2);
    }

    #[test]
    fn fill_syntax() {
        let vec: Vec<i32> = avec![42; 5];
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 5);
        assert_eq!(vec[0], 42);
        assert_eq!(vec[1], 42);
        assert_eq!(vec[2], 42);
        assert_eq!(vec[3], 42);
        assert_eq!(vec[4], 42);
    }
}
