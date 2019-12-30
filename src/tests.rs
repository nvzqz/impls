#![allow(dead_code)]

#[derive(Clone)]
struct Test;

#[derive(Clone)]
struct Box<T>(T);

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
        ($($a:ident, $b:ident, $c:ident, $d:ident;)+) => { $({
            const DOES_IMPL: bool = _bool::$a.value() | _bool::$b.value() ^ _bool::$c.value() & _bool::$d.value();

            assert_eq!(does_impl!(Test:   $a |  $b  ^  $c  & $d),   DOES_IMPL);
            assert_eq!(does_impl!(Test:   $a | ($b  ^ ($c  & $d))), DOES_IMPL);
            assert_ne!(does_impl!(Test: (($a |  $b) ^  $c) & $d),   DOES_IMPL);
        })+ };
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

#[test]
fn impls() {
    let mut errors = String::new();

    macro_rules! assert_impl {
        ($t:ty: $($trait_expr:tt)+) => {
            if !does_impl!($t: $($trait_expr)+) {
                errors.push_str(&format!(
                    "[{file}:{line}] {ty}: {expr}\n",
                    file = file!(),
                    line = line!(),
                    ty = stringify!($t),
                    expr = stringify!($($trait_expr)+)
                ));
            }
        };
    }

    assert_impl!(u8: (From<u16>) | (Into<u16>));
    assert_impl!((): (From<u8>) | (From<u16>) | Send);
    assert_impl!((): (!From<u8>) & !(From<u16>) & Send);
    assert_impl!((): Copy | Clone);
    assert_impl!((): Copy & Clone);
    assert_impl!((): !(Copy ^ Clone));
    assert_impl!(Test: Copy | Clone);
    assert_impl!(Test: !Copy | Clone);
    assert_impl!(Test: !Copy & Clone);
    assert_impl!(Test: !Copy & (Clone));
    assert_impl!(Test: !(Copy) & Clone);
    assert_impl!(Test: !(!Clone));
    assert_impl!(Test: !(Copy) & !(!Clone));
    assert_impl!(Test: !(Copy & Clone));
    assert_impl!(str: !Copy & !Clone);

    assert_impl!(Box<u8>: Clone);
    assert_impl!(Box<u8>: Clone & Send);
    assert_impl!(Box<u8>: !(From<u8> | Into<u8>));

    assert_impl!(&mut u8: !Copy);

    if !errors.is_empty() {
        panic!("Failed to satisfy implementations:\n{}", errors);
    }
}
