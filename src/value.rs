use backend::Backend;
/// A value in a function.
pub trait Value<'a> {
	fn new() -> Self;
}

pub trait Compilable<'a, B> where B: Backend<'a> {
	fn compile(&self, builder: B::Builder) -> B::Value;
}