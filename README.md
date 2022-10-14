# Typed macros
A traditional macro can't natively have arguments with type, they can only
accept a handful of meta types (`expr`, `ident`, `vis`...), with this crate
you can explicitely say the type of the argument you want the macro to take.

## Usage

Add the following dependency to your Cargo.toml file:

```
[dependencies]
typed_macros = "1.0.5"
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
	foo!(String::from("Some string")); // <- This won't throw an error.
	foo!(9u32); // <- This will throw an error.
}
```

The main macro is [`macrox`][macrox], it takes an input like `macro name(arg1: type1, arg2: type2) { /* Code */ }` Take a look to the [documentation][macrox] for a more in-depth approach to this macro. (Including multibranched macros!)

## Testing

You can run `cargo test` in the root directory, but you'll only see an error (an **intended** error) because the macro `this_should_warn` was asking for a `u32` type, and the test tried to use it with a `String`.

[macrox]: macro.macrox.html

## Contributing

Contributing is always welcomed, both in the form of code, documentation, ideas, etc... As the project is very simple there isn't a guide for contributing. If this is your first time contributing to an open source project maybe [this guide](https://github.com/firstcontributions/first-contributions) helps you.

## Stargazers

Thanks to all the people that starred the project, my monkey brain likes when number grow big.

[![Stargazers repo roster for @blyxyas/typed_macros](https://reporoster.com/stars/blyxyas/typed_macros)](https://github.com/blyxyas/typed_macros/stargazers)

## License

This software uses the MIT license. More info about this license is in the `LICENSE` file.