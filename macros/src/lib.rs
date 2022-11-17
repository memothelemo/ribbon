use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Token};

#[proc_macro]
pub fn impl_castable(tokens: TokenStream) -> TokenStream {
    pub struct MacroArgs {
        target: syn::Ident,
        comma: syn::token::Comma,
        subclasses: Vec<syn::Ident>,
        brace_token: syn::token::Brace,
    }

    impl syn::parse::Parse for MacroArgs {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let target = input.parse()?;
            let comma = input.parse()?;

            let content;
            let brace_token = syn::braced!(content in input);

            let subclasses: syn::punctuated::Punctuated<syn::Ident, Token![,]> =
                content.parse_terminated(syn::Ident::parse)?;

            let subclasses = subclasses.into_iter().collect();

            Ok(Self {
                target,
                comma,
                brace_token,
                subclasses,
            })
        }
    }

    #[allow(unused)]
    let MacroArgs {
        target,
        comma,
        subclasses,
        brace_token,
    } = parse_macro_input!(tokens as MacroArgs);

    let mut mut_transmutes = Vec::new();
    let mut transmutes = Vec::new();

    #[allow(unused)]
    for (i, subclass) in subclasses.iter().enumerate() {
        let mut inherited_classes = subclasses
            .get(..(i + 1))
            .map(|v| v.to_vec())
            .unwrap_or_default();

        if inherited_classes.is_empty() {
            transmutes.push(quote! {
                crate::instance::ClassName::#subclass => &crate::instance::utils::unsafe_transmute::<#target>(obj).base,
            });
            mut_transmutes.push(quote! {
                crate::instance::ClassName::#subclass => &mut crate::instance::utils::unsafe_transmute_mut::<#target>(obj).base,
            });
        } else {
            let mut transmute = quote! {
                crate::instance::utils::unsafe_transmute::<#target>(obj)
            };
            let mut mut_transmute = quote! {
                crate::instance::utils::unsafe_transmute_mut::<#target>(obj)
            };
            for class in inherited_classes {
                transmute = quote! {
                    #transmute.base
                };
                mut_transmute = quote! {
                    #mut_transmute.base
                };
            }
            transmutes.push(quote! {
                crate::instance::ClassName::#subclass => &#transmute,
            });
            mut_transmutes.push(quote! {
                crate::instance::ClassName::#subclass => &mut #mut_transmute,
            });
        }
    }

    quote! {
        impl crate::instance::InstanceCastable for #target {
            fn downcast<T: crate::instance::AnyInstance + crate::instance::DefaultClassName>(
                obj: &dyn crate::instance::AnyInstance
            ) -> Option<&T> {
                let default = T::default_class_name();
                if !obj.class().can_downcast(default) {
                    return None;
                }
                unsafe {
                    Some(crate::instance::utils::unsafe_transmute::<T>(match default {
                        crate::instance::ClassName::#target => obj,
                        #(#transmutes)*
                        _ => return None,
                    }))
                }
            }

            fn downcast_mut<T: crate::instance::AnyInstance + crate::instance::DefaultClassName>(
                obj: &mut dyn crate::instance::AnyInstance,
            ) -> Option<&mut T> {
                let default = T::default_class_name();
                if !obj.class().can_downcast(default) {
                    return None;
                }
                unsafe {
                    Some(crate::instance::utils::unsafe_transmute_mut::<T>(match default {
                        crate::instance::ClassName::#target => obj,
                        #(#mut_transmutes)*
                        _ => return None,
                    }))
                }
            }
        }
    }
    .into()
}
