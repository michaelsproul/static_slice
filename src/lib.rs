//! Just a single macro. See below.

/// Create a statically allocated slice of any type.
///
/// This macro allows you to define static slices in much the same way as static strings.
///
/// ```
/// #[macro_use]
/// extern crate static_slice;
///
/// fn main() {
///     let static_string: &'static str = "yay!";
///     let static_u8s: &'static [u8] = static_slice![u8: 1, 2, 3];
/// }
/// ```
///
/// This can be useful for, e.g. returning lots of static slices in a match.
#[macro_export]
macro_rules! static_slice {
    ($_type:ty: $($item:expr),*) => ({
        static STATIC_SLICE: &'static [$_type] = &[$($item),*];
        STATIC_SLICE
    });
}

#[cfg(test)]
mod test {
    #[allow(unused)]
    fn fn_return() -> &'static [u8] {
        static_slice![u8: 1, 2, 3]
    }

    #[test]
    fn multiple_slices() {
        let s1 = static_slice![u8: 1, 2, 3];
        let s2 = static_slice![u8: 1, 2, 3, 4];
        assert_eq!(s1, &s2[..3]);
    }
}
