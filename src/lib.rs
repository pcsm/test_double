extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::quote;

#[proc_macro]
pub fn test_doubles(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut output = TokenStream::new();

    functionlike_internal(&input.to_string(), &mut output);

    output.into()
}

fn functionlike_internal(input: &str, output: &mut TokenStream) {
    // Generate the AST from the token stream we were given
    let file: syn::File = syn::parse_str(input).expect("Failed to parse input");

    for item in file.items {
        process_single_item(item, None, output);
    }
}

/// Can be used like `#[test_double]` to use `____Mock` in tests or
/// `#[test_double(ObjectDummy)]` to use `ObjectDummy`.
#[proc_macro_attribute]
pub fn test_double(metadata: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut output = TokenStream::new();

    attribute_internal(&metadata.to_string(), &input.to_string(), &mut output);

    output.into()
}

fn attribute_internal(metadata: &str, input: &str, output: &mut TokenStream) {
    let mut alternate_ident = None;

    if !metadata.is_empty() {
        let error_message =
            "Invalid input to #[test_double] - use it like #[test_double(AlternateName)].";
        let meta: syn::Expr = syn::parse_str(metadata).expect(error_message);
        match meta {
            syn::Expr::Paren(expr_paren) => {
                let inner = expr_paren.expr;
                let inner = quote! { #inner };
                alternate_ident = Some(syn::Ident::new(&inner.to_string(), Span::call_site()));
            }
            _ => panic!(error_message),
        }
    }

    // Generate the AST from the token stream we were given
    let item: syn::Item = syn::parse_str(input).expect("Failed to parse input");

    process_single_item(item, alternate_ident, output);
}

fn process_single_item(item: syn::Item, alternate_ident: Option<syn::Ident>, output: &mut TokenStream) {
    match item {
        syn::Item::Use(mut use_original) => {
            // Make a copy of the original use statement
            let mut use_double = use_original.clone();

            modify_use_for_original(&mut use_original);
            modify_use_for_double(&mut use_double, alternate_ident);

            // Add the result to the back of our list of output tokens
            output.extend::<TokenStream>(quote!{
                #use_original
                #use_double
            }.into());
        }
        _ => panic!("Only use statements can be in the test_double! macro"),
    }
}

fn modify_use_for_original(use_original: &mut syn::ItemUse) {
    // Add `#[cfg(not(test))]` to our original use statement
    let not_test = quote! { (not(test)) };
    let cfg_not_test = syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        path: create_cfg_path(),
        tts: not_test.into(),
    };
    use_original.attrs.push(cfg_not_test);
}

fn modify_use_for_double(use_double: &mut syn::ItemUse, alternate_ident: Option<syn::Ident>) {
    // Add `#[cfg(test)]` to our test double use statement
    let test = quote! { (test) };
    let cfg_not_test = syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        path: create_cfg_path(),
        tts: test.into(),
    };
    use_double.attrs.push(cfg_not_test);

    modify_tree_for_double(&mut use_double.tree, alternate_ident);
}

// Change the name of the item used for the double use statement.
fn modify_tree_for_double(use_tree: &mut syn::UseTree, alternate_ident: Option<syn::Ident>) {
    match use_tree {
        syn::UseTree::Path(use_path) => {
            modify_tree_for_double(&mut use_path.tree, alternate_ident)
        },
        syn::UseTree::Name(use_name) => {
            // Change the imported name and add an "as" also
            // `use blah::Bar` => `use blah::BarMock as Bar`
            let original_ident = use_name.ident.clone();
            let default_ident = create_default_ident_for_double(&original_ident);
            let modified_ident = alternate_ident.unwrap_or(default_ident);

            let rename = syn::UseRename {
                ident: modified_ident,
                as_token: syn::token::As(Span::call_site()),
                rename: original_ident
            };
            *use_tree = syn::UseTree::Rename(rename);
        },
        syn::UseTree::Rename(use_rename) => {
            // Change the imported name
            // `use blah::Blah as Foo` => `use blah::BlahMock as Foo`
            let default_ident = create_default_ident_for_double(&use_rename.ident);
            use_rename.ident = alternate_ident.unwrap_or(default_ident);
        },
        syn::UseTree::Glob(_) => {
            panic!("test_double macros do not yet support * imports")
        },
        syn::UseTree::Group(_) => {
            panic!("test_double macros do not yet support imports groups")
        },
    }
}

fn create_default_ident_for_double(original_ident: &syn::Ident) -> syn::Ident {
    let name = quote! { #original_ident };
    syn::Ident::new(&format!("{}Mock", name), Span::call_site())
}

fn create_cfg_path() -> syn::Path {
    syn::Ident::new("cfg", Span::call_site()).into()
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

        let mut output = TokenStream::new();
        functionlike_internal(&input.to_string(), &mut output);

        assert_eq!(expected.to_string(), output.to_string());
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

        let mut output = TokenStream::new();
        attribute_internal("", &input.to_string(), &mut output);

        assert_eq!(expected.to_string(), output.to_string());
    }

    #[test]
    fn test_attribute_alternate_name() {
        let input = quote! {
            use quote::Tokens;
        };

        let expected = quote! {
            #[cfg(not(test))]
            use quote::Tokens;
            #[cfg(test)]
            use quote::TokensAlternate as Tokens;
        };

        let mut output = TokenStream::new();
        attribute_internal("(TokensAlternate)", &input.to_string(), &mut output);

        assert_eq!(expected.to_string(), output.to_string());
    }
}
