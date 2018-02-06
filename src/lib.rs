#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;

#[proc_macro]
pub fn test_double(input: TokenStream) -> TokenStream {
    let mut output = Tokens::new();
    test_double_internal(&input.to_string(), &mut output);

    // Turn that back into a token stream
    output.into()
}

fn test_double_internal(input: &str, output: &mut Tokens) {
    // Generate the AST from the token stream we were given
    let file: syn::File = syn::parse_str(input).expect("Failed to parse input");

    for item in file.items {
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
        let input = quote! {
            use quote::Tokens;
            use syn::Item;
        };
        // let mut first_prefix: syn::Punctuated<syn::Ident, syn::Colon2> = syn::Punctuated::new();
        // first_prefix.push(syn::Ident::from("quote"));
        // let first = syn::Item::Use(ItemUse {
        //     attrs: vec![],
        //     vis: syn::Visibility::Inherited,
        //     use_token: Token![use],
        //     leading_colon: None,
        //     prefix: first_prefix,
        //     tree: ,
        //     semi_token: Token![;]
        // });
        // let second = 6;
        // let input = vec![first, second];

        let expected = quote! {
            #[cfg(not(test))]
            use quote::Tokens;
            #[cfg(test)]
            use quote::TokensMock;

            #[cfg(not(test))]
            use syn::Item;
            #[cfg(test)]
            use syn::ItemMock;
        };

        let mut output = Tokens::new();
        test_double_internal(&input.to_string(), &mut output);

        assert_eq!(expected, output);
    }
}
