use backend::Backend;

/// An instruction builder.
pub trait Builder<'a, B> where B: Backend<'a> {
	fn new() -> Self;
	fn build_add(&self, a: B::Value, b: B::Value) -> B::Value;
	fn build_sub(&self, a: B::Value, b: B::Value) -> B::Value;
	fn build_ret(&self, v: Option<B::Value>);
}
