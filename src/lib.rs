//! # Typed macros
//! A traditional macro can't natively have arguments with type, they can only
//! accept a handful of meta types (`expr`, `ident`, `vis`...), with this crate
//! you can explicitely say the type of the argument you want the macro to take.
//!
//! ## Example
//!
//! ```rust
//! use typed_macros::macrox;
//!
//! macrox! {
//! 	/// You can even use attributes!
//! 	#[macro_export]
//! 	macro foo(bar: String) {
//! 		// Do something with bar...
//! 	}
//! }
//!
//! fn main() {
//! 	foo(String::from("Some string")) // <- This won't throw an error.
//! 	foo(9u32) // <- This will throw an error.
//! }
//! ```
//!
//! The main macro is [`macrox`][macrox], it takes an input like `macro name(arg1: type1, arg2: type2) { /* Code */ }`, both the [`macrox`][macrox]
//! 
//! [macrox]: macro.macrox.html

#![feature(macro_metavar_expr)]
#![warn(missing_docs)]

/// # Macrox
/// 
/// The main crate's macro, it takes a custom-syntax macro declaration. (`macro name(arg1: type1, arg2: type2 /* ... */) { /* Body */}`)
/// 
/// ## Example
/// 
/// ```rust
/// macrox! {
/// 	#[macro_export]
/// 	macro macro_name(arg: String) {
/// 		// Do something with arg.
/// 	}
/// }
/// 
/// fn main() {
/// 	// You can use the macro wherever you want.
/// 	macro_name!(String::from("Hi"));
/// }
/// ```
/// 
/// You can declare various macros inside `macrox!`, and they can have attributes.
#[macro_export(local_inner_macros)]
macro_rules! macrox {
	($($(#[$attr:meta])* macro $macro_name:ident($($arg: ident: $ty: ty), *) $body: block)*) => {
		$(
			$(#[$attr])
		*
		macro_rules! $macro_name {
			($$arg: expr) => {
				{
					$(let $arg: $ty = $$arg;)*
					$body
				}
			}
		}
		)*
	};
}

#[cfg(test)]
mod tests {
    use crate::macrox;
    #[test]
    pub fn it_works() {
        macrox! {
            macro some_name(y: String) {
                assert_eq!(y, String::from("hi"));
            }
        }

        // Now you can use it, and every time you use it, it will check the arg types.
        // (In compile-time!)

        some_name!(String::from("hi"));
    }

    pub fn it_warns() {
        macrox! {
            macro this_should_warn(var: u32) {
                // This will never get checked
                assert_eq!(var, 0);
            }
        }

        this_should_warn!("hi");
    }
}
