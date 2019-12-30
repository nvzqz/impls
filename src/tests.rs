#![allow(dead_code)]

struct Test;

trait True {}
impl<T: ?Sized> True for T {}
trait False {}

// Tests that `does_impl!` follows Rust's rules of precedence.
//
// Rust's rules of precedence are defined at:
// https://doc.rust-lang.org/reference/expressions.html#expression-precedence
#[test]
fn precedence() {
    use crate::_bool;

    macro_rules! table {
        ($($a:ident, $b:ident, $c:ident, $d:ident;)+) => {
            $(assert_eq!(
                does_impl!(Test: $a | $b ^ $c & $d),
                _bool::$a.value() | _bool::$b.value() ^ _bool::$c.value() & _bool::$d.value(),
            );)+
        };
    }

    // Table of cases where left-to-right parsing differs from precedence rules.
    //
    // https://play.rust-lang.org/?gist=7cbed02c68422f9464d0df79d39e99b0
    #[rustfmt::skip]
    table! {
        False, True,  False, False;
        False, True,  True,  False;
        True,  False, False, False;
        True,  False, True,  False;
        True,  False, True,  True;
        True,  True,  False, False;
        True,  True,  True,  False;
        True,  True,  True,  True;
    }
}
