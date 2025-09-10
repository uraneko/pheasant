use pheasant_macro_utils::{FallbackInscriptions, FallbackPlumber, FallbackPoet, Plumber, Poet};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn fail(attr: TokenStream, fun: TokenStream) -> TokenStream {
    let plumber = Plumber::<FallbackPlumber>::new(attr, fun).unwrap();
    let mut poet = Poet::<FallbackPoet>::new(plumber);

    poet.assemble().into()
}
