#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;

#[proc_macro]
pub fn test_doubles(input: TokenStream) -> TokenStream {
    let mut output = Tokens::new();

    functionlike_internal(&input.to_string(), &mut output);

    output.into()
}

fn functionlike_internal(input: &str, output: &mut Tokens) {
    // Generate the AST from the token stream we were given
    let file: syn::File = syn::parse_str(input).expect("Failed to parse input");

    for item in file.items {
       process_single_item(item, output); 
    }
}

/// Can be used like `#[test_double]` to use `____Mock` in tests or
/// `#[test_double(ObjectDummy)]` to use `ObjectDummy`.
#[proc_macro_attribute]
pub fn test_double(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = Tokens::new();

    attribute_internal(&metadata.to_string(), &input.to_string(), &mut output);

    output.into()
}

fn attribute_internal(metadata: &str, input: &str, output: &mut Tokens) {
    if !metadata.is_empty() {
        let meta: syn::TypeParen = syn::parse_str(metadata).expect("Invalid input to #[test_double] - use it like #[test_double(AlternateName)].");
        // match meta.elem {

        // }
    }

    // Generate the AST from the token stream we were given
    let item: syn::Item = syn::parse_str(input).expect("Failed to parse input");

    process_single_item(item, output);
}

fn process_single_item(item: syn::Item, output: &mut Tokens) {
    match item {
        syn::Item::Use(mut use_original) => {
            // Make a copy of the original use statement
            let mut use_double = use_original.clone();

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
            use_double.attrs.push(cfg_not_test);

            // Change the name of the item used for the double use statement.
            // `use blah::Bar` => `use blah::BarMock as Bar`
            // `use blah::Blah as Foo` => `use blah::BlahMock as Foo`
            match &mut use_double.tree {
                &mut syn::UseTree::Path(ref mut use_path) => {
                    // Change the imported name
                    let ident = use_path.ident;
                    let name = quote! { #ident };
                    let double_name = syn::Ident::from(format!("{}Mock", name));
                    use_path.ident = double_name;

                    // If we don't have a rename set up already, add one back
                    // to the original name.
                    if use_path.rename.is_none() {
                        use_path.rename = Some((Default::default(), ident));
                    }
                },
                &mut syn::UseTree::Glob(_) => panic!("test_double macros do not yet support * imports"),
                &mut syn::UseTree::List(_) => panic!("test_double macros do not yet support imports lists")
            }

            // Add the result to the back of our list of output tokens
            output.append_all(quote!{
                #use_original
                #use_double
            });
        },
        _ => panic!("Only use statements can be in the test_double! macro")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functionlike_basic() {
        let input = quote! {
            use quote::Tokens;
            use syn::Item;
        };

        let expected = quote! {
            #[cfg(not(test))]
            use quote::Tokens;
            #[cfg(test)]
            use quote::TokensMock as Tokens;

            #[cfg(not(test))]
            use syn::Item;
            #[cfg(test)]
            use syn::ItemMock as Item;
        };

        let mut output = Tokens::new();
        functionlike_internal(&input.to_string(), &mut output);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_attribute_rename() {
        let input = quote! {
            use quote::Tokens as SomethingElse;
        };

        let expected = quote! {
            #[cfg(not(test))]
            use quote::Tokens as SomethingElse;
            #[cfg(test)]
            use quote::TokensMock as SomethingElse;
        };

        let mut output = Tokens::new();
        attribute_internal("", &input.to_string(), &mut output);

        assert_eq!(expected, output);
    }
}
