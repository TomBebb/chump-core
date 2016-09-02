use builder::Builder;
use types::Type;
use value::Value;

/// A compilation backend.
pub trait Backend<'a>: Sized {
	type Label;
	type Builder: Builder<'a, Self>;
	type Type: Type<'a, Self>;
	type Value: Value<'a>;
	fn new() -> Self;
}