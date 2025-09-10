use proc_macro::TokenStream;

use pheasant_macro_utils::{
    Method, Plumber, Poet, ProcessInscriptions, ProcessPlumber, ProcessPoet,
};

#[proc_macro_attribute]
pub fn post(attr: TokenStream, fun: TokenStream) -> TokenStream {
    let plumber = Plumber::<ProcessPlumber>::new(Method::Post, attr, fun).unwrap();
    let mut poet = Poet::<ProcessPoet>::new(plumber);

    poet.assemble().into()
}
