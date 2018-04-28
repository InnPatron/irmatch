#[macro_export]
macro_rules! irmatch {
    ($v: expr; $( $p: pat => $r: expr ),+) => {{
        match $v {
            $($p => $r,)+

            _ => {
                unreachable!("Failed to match {} at {}:{}",
                               stringify!($($p)+), file!(), line!())
            }
        }

    }}
}

#[cfg(test)]
mod tests {
    enum T {
        A(i32),
        B(i32, i32),
        C(i32, bool),
        D,
        E {
            f1: i32,
            f2: bool
        }
    }

    #[test]
    fn test_single_tuple() {
        let t = T::A(5);
        let v = irmatch!(t; T::A(i) => i);
        assert_eq!(5, v);
    }

    #[test]
    fn test_double_tuple() {
        let t = T::B(5, -7);
        let v = irmatch!(t; T::B(i1, i2) => (i1, i2));
        assert_eq!((5, -7), v);
    }

    #[test]
    fn test_struct() {
        let t = T::E {
            f1: 10,
            f2: false
        };
        let v = irmatch!(t; T::E{f1: i, f2: b} => (i, b));
        assert_eq!((10, false), v);
    }

    #[test]
    fn test_empty() {
        let t = T::D;
        let v = irmatch!(t; T::D => ());
        assert_eq!((), v);
    }

    #[test]
    #[should_panic]
    fn test_invariant_error() {
        let t = T::B(5, 10);
        let v = irmatch!(t; T::D => ());

        let t = T::A(5);
        let v = irmatch!(t; T::E{..} => ());
    }

    #[test]
    fn multi_pattern() {
        let t = T::B(5, 10);
        let v = irmatch!(t; T::B(..) => (), T::A(..) => ());

        assert_eq!((), v);
    }

    #[test]
    #[should_panic]
    fn multi_pattern_fail() {
        let t = T::B(5, 10);
        let v = irmatch!(t; T::D => (), T::A(..) => ());

        assert_eq!((), v);
    }
}
