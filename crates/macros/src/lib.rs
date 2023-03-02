use std::collections::VecDeque;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, token::Brace, Ident, Token};

/**
 * BaseInstance {
 *  WorldRoot {
 *    Hello,
 *  },
 * },
 *
 */

struct TreeInput {
    name: Ident,
    _braces: Option<Brace>,
    fields: Option<Punctuated<TreeInput, Token![,]>>,
}

impl Parse for TreeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = Ident::parse(input)?;
        let content;
        let brace = match syn::__private::parse_braces(input) {
            Ok(braces) => {
                content = braces.content;
                braces.token
            }
            Err(..) => {
                return Ok(TreeInput {
                    name,
                    _braces: None,
                    fields: None,
                });
            }
        };

        let mut fields = Punctuated::new();
        while !content.is_empty() {
            if let Ok(indent) = TreeInput::parse(&content) {
                fields.push(indent);
            }
            if content.is_empty() {
                break;
            }
            let punct: Token![,] = content.parse()?;
            fields.push_punct(punct);
        }

        Ok(TreeInput {
            name,
            _braces: Some(brace),
            fields: Some(fields),
        })
    }
}

#[proc_macro]
pub fn class_name_tree(input: TokenStream) -> TokenStream {
    let tree: TreeInput = parse_macro_input!(input);

    // visit trees and get their class names
    let mut variants: Vec<Ident> = Vec::new();
    fn visit_tree_1(tree: &TreeInput, variants: &mut Vec<Ident>) {
        variants.push(tree.name.clone());
        if let Some(fields) = tree.fields.as_ref() {
            for pair in fields.pairs() {
                visit_tree_1(pair.value(), variants);
            }
        }
    }
    visit_tree_1(&tree, &mut variants);

    // enum declaration
    let enum_declaration = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString)]
        #[allow(clippy::all)]
        pub enum ClassName {
            #(#variants,)*
        }
    };

    // partialord implementation
    let mut tokens: Vec<quote::__private::TokenStream> = Vec::new();
    let mut stack: VecDeque<Ident> = VecDeque::new();

    let fields = tree.fields.expect("empty fields");
    fn visit_tree_2(
        current: &TreeInput,
        tokens: &mut Vec<quote::__private::TokenStream>,
        stack: &mut VecDeque<Ident>,
    ) {
        if let Some(fields) = current.fields.as_ref() {
            for pair in fields.pairs() {
                let pair = pair.value();
                stack.push_front(pair.name.clone());
                visit_tree_2(pair, tokens, stack);
                stack.pop_front();
            }
        }

        let mut iter = stack.iter();
        iter.next();

        let mut initial_tokens = Vec::new();
        for ident in iter {
            initial_tokens.push(quote! {
                Self::#ident,
            });
        }
        let name = current.name.clone();
        tokens.push(quote! {
            Self::#name => &[ #(#initial_tokens)* ],
        });
    }

    stack.push_front(tree.name);

    for pair in fields {
        stack.push_front(pair.name.clone());
        visit_tree_2(&pair, &mut tokens, &mut stack);
        stack.pop_front();
    }

    let base_classes_impl = quote! {
        #[allow(clippy::all)]
        impl ClassName {
            pub fn base_classes(&self) -> &'static [Self] {
                match self {
                    #(#tokens)*
                    _ => &[],
                }
            }
        }
    };

    // let partial_eq_impl = quote! {
    //     #[allow(clippy::all)]
    //     impl PartialOrd for ClassName {
    //         fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    //             if self == other {
    //                 return Some(std::cmp::Ordering::Equal);
    //             }
    //             match (self, other) {
    //                 #(#tokens)*
    //                 _ => return None,
    //             }
    //             Some(std::cmp::Ordering::Greater)
    //         }
    //     }
    // };

    quote! {
        #enum_declaration
        #base_classes_impl
        // #partial_eq_impl
    }
    .into()
}
