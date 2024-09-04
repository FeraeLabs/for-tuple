//! Procedural macro that calls another macro with tuples of specified lengths.

mod for_tuple;

use syn::parse_macro_input;

use crate::for_tuple::ForTupleInput;

/// Calls another macro with tuples of specified lengths.
///
/// This is useful to implement for tuple types.
///
/// The callback macro will be passed a comma-separated list of
///
/// ```no_rust
/// $index:tt => $name:ident : $ty:ident
/// ```
///
/// where:
///
/// - `$index` is the index for that entry, to be used for field access.
/// - `$name` is a generated identifier of form `_$index`, to be used as
///   variables.
/// - `$ty` is the type of that entry.
///
/// # Example
///
/// ```
/// # use for_tuple::for_tuple;
/// # trait Foo { fn foo(&self); }
/// macro_rules! impl_tuple {
///     ($($index:tt => $name:ident : $ty:ident),*) => {
///         impl<$($ty),*> Foo for ($($ty,)*) {
///             fn foo(&self) {
///                 $(
///                     let $name = &self.$index;
///                 )*
///                 // do something with variables
///             }
///         }
///         // do stuff here
///     }
/// }
///
/// // Implements Foo for tuples of size 1 to 8 (inclusive).
/// for_tuple!(impl_tuple! for 1..=8);
/// ```
///
/// Note that the `impl` is on the type `($($ty,))` with the comma inside, such
/// that `(T,)` is a tuple.
#[proc_macro]
pub fn for_tuple(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ForTupleInput);
    match crate::for_tuple::for_tuple(input) {
        Ok(output) => output.into(),
        Err(e) => e.write_errors().into(),
    }
}
