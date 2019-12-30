//! Determine if a type does implement a logical trait expression.

#![deny(missing_docs)]

#[doc(hidden)]
pub extern crate core as _core;

/// Type-level booleans.
///
/// This module and its contents are not for public consumption and are thus
/// exempt from semantic versioning.
#[doc(hidden)]
#[path = "bool.rs"]
pub mod _bool;

/// Returns `true` if a type does implement a logical trait expression.
#[macro_export(local_inner_macros)]
macro_rules! does_impl {
    ($type:ty: $($trait_expr:tt)+) => {
        _does_impl!($type: $($trait_expr)+)
    };
}

/// Handles the dirty work of `does_impl`.
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! _does_impl {
    // ONE: Turn `$trait` into `true` or `false` based on whether `$type`
    // implements it.
    ($type:ty: $trait:path) => {{
        // Do not import types in order to prevent trait name collisions.

        /// Fallback trait with `False` for `DOES_IMPL` if the type does not
        /// implement the given trait.
        trait DoesNotImpl {
            const DOES_IMPL: $crate::_bool::False = $crate::_bool::False;
        }
        impl<T: ?Sized> DoesNotImpl for T {}

        /// Concrete type with `True` for `DOES_IMPL` if the type does implement
        /// the given trait. Otherwise, it falls back to `DoesNotImpl`.
        struct Wrapper<T: ?Sized>($crate::_core::marker::PhantomData<T>);

        #[allow(dead_code)]
        impl<T: ?Sized + $trait> Wrapper<T> {
            const DOES_IMPL: $crate::_bool::True = $crate::_bool::True;
        }

        <Wrapper<$type>>::DOES_IMPL.value()
    }};

    // PAREN+OR
    ($type:ty: ($($t1:tt)+) | $($t2:tt)+) => {
        _does_impl!($type: $($t1)+)
        |
        _does_impl!($type: $($t2)+)
    };
    // PAREN+OR+NOT
    ($type:ty: !($($t1:tt)+) | $($t2:tt)+) => {
        !_does_impl!($type: $($t1)+)
        |
        _does_impl!($type: $($t2)+)
    };
    // PAREN+AND
    ($type:ty: ($($t1:tt)+) & $($t2:tt)+) => {
        _does_impl!($type: $($t1)+)
        &
        _does_impl!($type: $($t2)+)
    };
    // PAREN+AND+NOT
    ($type:ty: !($($t1:tt)+) & $($t2:tt)+) => {
        !_does_impl!($type: $($t1)+)
        &
        _does_impl!($type: $($t2)+)
    };
    // PAREN+XOR
    ($type:ty: ($($t1:tt)+) ^ $($t2:tt)+) => {
        _does_impl!($type: $($t1)+)
        ^
        _does_impl!($type: $($t2)+)
    };
    // PAREN+XOR+NOT
    ($type:ty: !($($t1:tt)+) ^ $($t2:tt)+) => {
        !_does_impl!($type: $($t1)+)
        ^
        _does_impl!($type: $($t2)+)
    };

    // OR: Any.
    ($type:ty: $t1:path | $($t2:tt)+) => {{
        _does_impl!($type: $t1)
        |
        _does_impl!($type: $($t2)+)
    }};
    // OR+NOT: Any.
    ($type:ty: !$t1:path | $($t2:tt)+) => {{
        !_does_impl!($type: $t1)
        |
        _does_impl!($type: $($t2)+)
    }};

    // AND: 0 lifetimes, 0 generics.
    ($type:ty: $t1:ident & $($t2:tt)+) => {{
        _does_impl!($type: $t1)
        &
        _does_impl!($type: $($t2)+)
    }};
    // AND+NOT: 0 lifetimes, 0 generics.
    ($type:ty: !$t1:ident & $($t2:tt)+) => {{
        !_does_impl!($type: $t1)
        &
        _does_impl!($type: $($t2)+)
    }};

    // AND: 1+ lifetimes, 0+ generics.
    (
        $type:ty:
        $t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        &
        $($t2:tt)+
    ) => {{
        _does_impl!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >)
        &
        _does_impl!($type: $($t2)+)
    }};
    // AND+NOT: 1+ lifetimes, 0+ generics.
    (
        $type:ty:
        !$t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        &
        $($t2:tt)+
    ) => {{
        !_does_impl!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >)
        &
        _does_impl!($type: $($t2)+)
    }};

    // AND: 0 lifetimes, 1+ generics.
    (
        $type:ty:
        $t1:ident < $($t1_generic:ty),+ $(,)? >
        &
        $($t2:tt)+
    ) => {{
        _does_impl!($type: $t1 < $($t1_generic),+ >)
        &
        _does_impl!($type: $($t2)+)
    }};
    // AND+NOT: 0 lifetimes, 1+ generics.
    (
        $type:ty:
        !$t1:ident < $($t1_generic:ty),+ $(,)? >
        &
        $($t2:tt)+
    ) => {{
        !_does_impl!($type: $t1 < $($t1_generic),+ >)
        &
        _does_impl!($type: $($t2)+)
    }};

    // XOR: 0 lifetimes, 0 generics.
    ($type:ty: $t1:ident ^ $($t2:tt)+) => {{
        _does_impl!($type: $t1)
        ^
        _does_impl!($type: $($t2)+)
    }};
    // XOR+NOT: 0 lifetimes, 0 generics.
    ($type:ty: !$t1:ident ^ $($t2:tt)+) => {{
        ! _does_impl!($type: $t1)
        ^
        _does_impl!($type: $($t2)+)
    }};

    // XOR: 1+ lifetimes, 0+ generics.
    (
        $type:ty:
        $t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        ^
        $($t2:tt)+
    ) => {{
        _does_impl!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >)
        ^
        _does_impl!($type: $($t2)+)
    }};
    // XOR+NOT: 1+ lifetimes, 0+ generics.
    (
        $type:ty:
        ! $t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        ^
        $($t2:tt)+
    ) => {{
        !_does_impl!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >)
        ^
        _does_impl!($type: $($t2)+)
    }};

    // XOR: 0 lifetimes, 1+ generics.
    (
        $type:ty:
        $t1:ident < $($t1_generic:ty),+ $(,)? >
        ^
        $($t2:tt)+
    ) => {{
        _does_impl!($type: $t1 < $($t1_generic),+ >)
        ^
        _does_impl!($type: $($t2)+)
    }};
    // XOR+NOT: 0 lifetimes, 1+ generics.
    (
        $type:ty:
        ! $t1:ident < $($t1_generic:ty),+ $(,)? >
        ^
        $($t2:tt)+
    ) => {{
        ! _does_impl!($type: $t1 < $($t1_generic),+ >)
        ^
        _does_impl!($type: $($t2)+)
    }};
}

// Declare after macros in order to be able to use them.
#[cfg(test)]
mod tests;
