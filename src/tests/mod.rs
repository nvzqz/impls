#![allow(dead_code)]

mod bool;

#[derive(Clone)]
struct Test;

#[derive(Clone)]
struct Box<T>(T);

trait True {}
impl<T: ?Sized> True for T {}
trait False {}

// Tests that `impls!` follows Rust's rules of precedence.
//
// Rust's rules of precedence are defined at:
// https://doc.rust-lang.org/reference/expressions.html#expression-precedence
#[test]
fn precedence() {
    macro_rules! table {
        ($($a:ident, $b:ident, $c:ident, $d:ident;)+) => { $({
            const IMPLS: bool = bool::$a.value() | bool::$b.value() ^ bool::$c.value() & bool::$d.value();

            assert_eq!(impls!(Test:   $a |  $b  ^  $c  & $d),   IMPLS);
            assert_eq!(impls!(Test:   $a | ($b  ^ ($c  & $d))), IMPLS);
            assert_ne!(impls!(Test: (($a |  $b) ^  $c) & $d),   IMPLS);
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
            if !impls!($t: $($trait_expr)+) {
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
