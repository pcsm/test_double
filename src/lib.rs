#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;

#[proc_macro]
pub fn test_double(input: TokenStream) -> TokenStream {
    // Generate the AST from the token stream we were given
    let file: syn::File = syn::parse(input).expect("Failed to parse input");

    // Actually process the input
    let mut output = Tokens::new();
    internal(file.items, &mut output);

    // Turn that Rust back into a token stream
    output.into()
}

fn internal(items: Vec<syn::Item>, output: &mut Tokens) {
    for item in items {
        match item {
            // syn::Item::Use(use_item) => {
            item @ syn::Item::Use(_) => {
                output.append_all(quote!{
                    #item
                });
            },
            _ => panic!("Only use statements can be in the test_double! macro")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
    }
}
