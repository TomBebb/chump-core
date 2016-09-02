use backend::Backend;
use std::fmt;

#[derive(Copy, Clone, Debug)]
/// A lightweight way of representing a category of type.
pub enum TypeKind {
	Integer,
	Real,
	Function,
	Pointer,
	Struct
}

/// This represents a single type in the compiler.
pub trait Type<'a, B>: Clone where B: Backend<'a> {
	/// Make a new pointer type with a reference to `elem`.
	fn new_pointer(backend: &B, elem: Self) -> Self;
	/// Make a new structure type with the fields `fields`.
	fn new_struct(backend: &B, fields: &[Self]) -> Self;
	fn new_signature(backend: &B, args: &[Self], ret: Self) -> Self;
	/// Make a new integer type capable of holding up to `nbits` bits.
	fn new_int(backend: &B, nbits: usize, signed: bool) -> Self;
	/// Make a new real (floating-point) type capable of holding up to `nbits` bits.
	fn new_real(backend: &B, nbits: usize) -> Self;

	fn get_args(&self) -> Option<Vec<Self>>;
	fn get_return(&self) -> Option<Self>;
	/// Get the size of this type in bytes.
	fn get_size(&self) -> usize;
	/// Get the size of this type in bits.
	#[inline(always)]
	fn get_num_bits(&self) -> usize {
		self.get_size() / 8
	}

	/// Get the category of type this falls into.
	fn get_kind(&self) -> TypeKind;
}

pub trait Typeable<'a, B> where B: Backend<'a> {
	fn get_type(backend: &B) -> B::Type;
}
macro_rules! typeable {
	(plain, $ty: ty, $be:ident, $init:expr) => (
impl<'a, B> Typeable<'a, B> for $ty where B: Backend<'a> {
	fn get_type($be: &B) -> B::Type {
		$init
	}
}
);
    (int, $ty:ty, $nbits:expr, $signed:expr) => (
    	typeable!{plain, $ty, a, Type::new_int(a, $nbits, $signed)}
    );
    (struct, $($id:ident),+) => (
impl<'a, Back, $($id),+> Typeable<'a, Back> for ($($id,)+ ) where $($id:Typeable<'a, Back>),+, Back: Backend<'a> {
	fn get_type(backend: &Back) -> Back::Type {
		Type::new_struct(backend, &[$($id::get_type(backend)),+])
	}
}
    );
}
typeable!{int, bool, 1, false}
typeable!{int, i8, 8, true}
typeable!{int, u8, 8, false}
typeable!{int, i16, 16, true}
typeable!{int, u16, 16, false}
typeable!{int, i32, 32, true}
typeable!{int, u32, 32, false}
typeable!{int, i64, 64, true}
typeable!{int, u64, 64, false}
typeable!{plain, f32, be, Type::new_real(be, 32)}
typeable!{plain, f64, be, Type::new_real(be, 64)}
typeable!{struct, A}
typeable!{struct, A, B}
typeable!{struct, A, B, C}
typeable!{struct, A, B, C, D}
typeable!{struct, A, B, C, D, E}
typeable!{struct, A, B, C, D, E, F}
typeable!{struct, A, B, C, D, E, F, G}