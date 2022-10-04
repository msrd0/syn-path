#![warn(rust_2018_idioms, unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![forbid(unsafe_code)]

//! This crate contains a macro to turn a path into a `syn::Path` at compile time.

#[doc(hidden)]
pub mod private {
	pub use core::{
		default::Default,
		option::Option::{None, Some}
	};
	pub use proc_macro2::{Ident, Span};
	pub use std::stringify;
	pub use syn::{punctuated::Punctuated, Path, PathSegment};
}

/// A simple macro that can take paths of the form `my_crate::my_mod::FooBar` and
/// `::my_crate::my_mod::FooBar` and turn them into a `syn::Path`.
#[macro_export]
macro_rules! path {
	(:: $($segment:ident)::*) => {
		$crate::private!($crate::private::Some($crate::private::Default::default()), $($segment),*)
	};
	($($segment:ident)::*) => {
		$crate::private!($crate::private::None, $($segment),*)
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! private {
	($leading_colon:expr, $($segment:ident),*) => {
		{
			#[allow(unused_mut)]
			let mut segments: $crate::private::Punctuated<$crate::private::PathSegment, _> = $crate::private::Default::default();
			$(
				segments.push($crate::private::PathSegment {
					ident: $crate::private::Ident::new(
						$crate::private::stringify!($segment),
						$crate::private::Span::call_site()
					),
					arguments: $crate::private::Default::default()
				});
			)*
			$crate::private::Path {
				leading_colon: $leading_colon,
				segments
			}
		}
	};
}

#[cfg(test)]
mod tests {
	use syn::Path;

	#[test]
	fn with_leading_colon() {
		let path = path!(::my_crate::my_mod::FooBar);
		let expected: Path = syn::parse_str("::my_crate::my_mod::FooBar").unwrap();
		assert_eq!(expected, path);
	}

	#[test]
	fn without_leading_colon() {
		let path = path!(my_crate::my_mod::FooBar);
		let expected: Path = syn::parse_str("my_crate::my_mod::FooBar").unwrap();
		assert_eq!(expected, path);
	}
}
