#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
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
            syn::Item::Use(mut use_original) => {
                // Make a copy of the original `use blah::Blah;`
                let mut use_mock = use_original.clone();


                // let cfg = quote! { cfg };
                // let cfg: syn::Path = syn::parse(cfg.into()).unwrap();
                let cfg: syn::Path = syn::Ident::from("cfg").into();

                // Add `#[cfg(not(test))]` to our original use statement
                let not_test = quote! { (not(test)) };
                let cfg_not_test = syn::Attribute {
                    pound_token: Default::default(),
                    style: syn::AttrStyle::Outer,
                    bracket_token: Default::default(),
                    path: cfg.clone(),
                    tts: not_test.into(),
                    is_sugared_doc: false
                };
                use_original.attrs.push(cfg_not_test);

                // Add `#[cfg(test)]` to our test double use statement
                let test = quote! { (test) };
                let cfg_not_test = syn::Attribute {
                    pound_token: Default::default(),
                    style: syn::AttrStyle::Outer,
                    bracket_token: Default::default(),
                    path: cfg,
                    tts: test.into(),
                    is_sugared_doc: false
                };
                use_mock.attrs.push(cfg_not_test);

                // Change the name of the item used

                // Add the result to the back of our list of output tokens
                output.append_all(quote!{
                    #use_original
                    #use_mock
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
