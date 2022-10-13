# Typed macros
A traditional macro can't natively have arguments with type, they can only
accept a handful of meta types (`expr`, `ident`, `vis`...), with this crate
you can explicitely say the type of the argument you want the macro to take.

## Usage

Add the following dependency to your Cargo.toml file:

```
[dependencies]
typed_macros = "1.0.4"
```

And check the [documentation](https://docs.rs/typed_macros/latest/typed_macros/) on how to use it.

## Example

```rust
#![feature(macro_metavar_expr)]
use typed_macros::macrox;

macrox! {
	/// You can even use attributes!
	#[macro_export]
	macro foo(bar: String) {
		// Do something with bar...
	}
}

fn main() {
	foo(String::from("Some string")); // <- This won't throw an error.
	foo(9u32); // <- This will throw an error.
}
```

The main macro is [`macrox`][macrox], it takes an input like `macro name(arg1: type1, arg2: type2) { /* Code */ }` Take a look to the [documentation][macrox] for a more in-depth approach to this macro. (Including multibranched macros!)



## Testing

You can run `cargo test` in the root directory, but you'll only see an error (an **intended** error) because the macro `this_should_warn` was asking for a `u32` type, and the test tried to use it with a `String`.

[macrox]: macro.macrox.html
