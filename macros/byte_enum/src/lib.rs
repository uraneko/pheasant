use proc_macro::TokenStream;

// TODO byte_repr attr macro
// HttpSocket {
//  proto: u8,
// }
//
// #[byte_repr]
// #[target(HttpSocket = proto)]
// enum Protocol { H1, H2 }
//
// generates
// let vars_count: usize = Protocol.count_variants();
// let byte_type = byte_type(vars_count);
// let bits = variants_as_bytes(vars_count);
//
// quote! {
// impl From<&Protocol> for #byte_type {
//  fn from(p: &Protocol) -> Self {
//      match p {
//          #(Protocol::#proto => #bits)*,
//      }
//  }
// }
// }
//
// impl HttpSocket {
//  fn h1(h1: bool) {
//      if h1 { self.proto | H1.into() } else { self.proto & !H1.into() }
//  }
// }
//

#[proc_macro_attribute]
pub fn byte_enum(attr: TokenStream, tree: TokenStream) -> TokenStream {
    tree
}
