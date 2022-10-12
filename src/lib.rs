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
//! 	foo!(String::from("Some string")) // <- This won't throw an error.
//! 	// foo!(9u32) // <- This will throw an error.
//! }
//! ```
//!
//! The main macro is [`macrox`][macrox], it takes an input like `macro
//! name(arg1: type1, arg2: type2) { /* Code */ }`, both the [`macrox`][macrox]
//!
//! [macrox]: macro.macrox.html

#![feature(macro_metavar_expr)]
#![warn(missing_docs)]

/// # Macrox
///
/// The main crate's macro, it takes a custom-syntax macro declaration. (`macro
/// name(arg1: type1, arg2: type2 /* ... */) { /* Body */}`)
///
/// ## Example
///
/// ```rust
/// use typed_macros::macrox;
/// 
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
/// You can declare various macros inside `macrox!`, and they can have
/// attributes.
///
/// ## Single-branched and Multi-branched macros
///
/// With this macro you can write both single-branched and multi-branched
/// macros, and both are really easy!
///
/// ### Single-branched macros
///
/// These are macros like the one in the example, with just one possible branch.
///
/// ### Multi-branched macros
///
/// These are a little bit more complicated, they use identifiers both
/// distinguishing what branch you're trying to use.
///
/// An identifier is anything that starts with '@', e. g. `@a`
/// Also, between the two branches there must be a ';'
/// #### Example
///
/// ```rust
/// use typed_macros::macrox;
///
/// macrox! {
/// 	#[macro_export]
/// 	macro my_macro(@a x: String) {
/// 		// Do something with x being a String
/// 	};
///
/// 	(@b x: u32) {
/// 		// Do something with x being a u32
/// 	}
/// }
///
/// fn main() {
/// 	my_macro!(@a String::new("hi!"));
/// 	my_macro!(@b 5_u32);
/// }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! macrox {
	($($(#[$attr:meta])* macro $macro_name:ident$(($($arg: ident: $ty: ty), *) $body: block); +)*) => {
		$(
			$(#[$attr])*
		macro_rules! $macro_name {
				$(
				($$arg: expr) => {
					{
						$(let $arg: $ty = $$arg;)*
						$body
					}
				};
				)+
			}
		)*
	};

	// Multibranch with identifiers
	($($(#[$attr:meta])* macro $macro_name:ident$(($(@$identifier: ident $arg: ident: $ty: ty), *) $body: block); +)*) => {
		$(
			$(#[$attr])*
		macro_rules! $macro_name {
				$(
				($(@$identifier)* $$arg: expr) => {
					{
						$(let $arg: $ty = $$arg;)*
						$body
					}
				};
				)+
			}
		)*
	};
}

#[cfg(test)]
mod tests {
    use crate::macrox;
    #[test]
    pub fn singlebranched() {
        macrox! {
            macro some_name(y: String) {
                assert_eq!(y, String::from("hi"));
            }
        }

        // Now you can use it, and every time you use it, it will check the arg types.
        // (In compile-time!)
        some_name!(String::from("hi"));
    }

    #[test]
    fn multibranched() {
        macrox! {
                macro some_name

                (@a y: String) {
                    assert_eq!(y, String::from("hi"));
                };

                (@b x: u32) {
                    assert_eq!(x, 5u32);
                }

            // Now you can use it, and every time you use it, it will check the arg types.
            // (In compile-time!)
        }
        some_name!(@a String::from("hi"));
        some_name!(@b 5u32);
    }

    // pub fn it_warns() {
    //     macrox! {
    //         macro this_should_warn(var: u32) {
    //             // This will never get checked
    //             assert_eq!(var, 0);
    //         }
    //     }

    //     this_should_warn!("hi");
    // }
}
