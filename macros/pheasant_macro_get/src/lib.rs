use pheasant_macro_utils::{
    Method, Plumber, Poet, ProcessInscriptions, ProcessPlumber, ProcessPoet,
};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn get(attr: TokenStream, fun: TokenStream) -> TokenStream {
    let plumber = Plumber::<ProcessPlumber>::new(Method::Get, attr, fun).unwrap();
    let mut poet = Poet::<ProcessPoet>::new(plumber);

    poet.assemble().into()
}
