#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro]
pub fn test_double(input: TokenStream) -> TokenStream {
    // Generate the AST from the token stream we were given
    let item: syn::Item = syn::parse(input).expect("Failed to parse input");

    match item {
        syn::Item::Use(use_item) => {
            unimplemented!()
        },
        _ => panic!("Only use statements can be in the test_double! macro")
    }

    let output = quote!{
        #item
    };

    // Turn that Rust back into a token stream
    output.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
