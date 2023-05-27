#[cfg(not(target_family = "wasm"))]
#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn input_system(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    inner::input_system_core(args.into(), input.into()).into()
}

#[cfg(not(target_family = "wasm"))]
mod inner {
    use proc_macro2::TokenStream;
    use proc_macro_error::abort;
    use quote::quote;
    use syn::{parse2, FnArg, ItemFn};

    pub(crate) fn input_system_core(_: TokenStream, input: TokenStream) -> TokenStream {
        let item_fn = match parse2::<ItemFn>(input) {
            Ok(syntax_tree) => syntax_tree,
            Err(error) => return error.to_compile_error(),
        };

        let name = item_fn.sig.ident;

        let inputs: Vec<_> = item_fn
            .sig
            .inputs
            .iter()
            .map(|input| match input {
                FnArg::Receiver(_) => abort!(input, "Input function cannot have self"),
                FnArg::Typed(arg) => {
                    let arg_name = match arg.pat.as_ref() {
                        syn::Pat::Ident(ident) => &ident.ident,
                        _ => abort!(arg, "Expected Ident?"),
                    };
                    let arg_type = match arg.ty.as_ref() {
                        syn::Type::Reference(reference) => match &reference.elem.as_ref() {
                            syn::Type::Path(path) => path
                                .path
                                .segments
                                .last()
                                .unwrap_or_else(|| abort!(path.path, "What?")),
                            _ => abort!(reference.elem, "Expected a Component"),
                        },
                        _ => abort!(arg.ty, "Expected the parameter type to be a reference"),
                    };
                    (arg_name, arg_type)
                }
            })
            .collect();

        let types = inputs.iter().map(|(_, t)| t);
        let names = inputs.iter().map(|(n, _)| n);

        let body = item_fn.block;

        let output = quote! {

            fn #name(mut query: bevy::prelude::Query<(&mut bevy_utility_ai::AIMeta #(, &#types)*)>) {
                let key = #name as usize;
                for (mut ai_meta #(, #names)*) in query.iter_mut() {
                    let score = #body;
                    let mut entry = ai_meta.input_scores.entry(key).or_insert(f32::NEG_INFINITY);
                    *entry = score;
                }
            }
        };

        output.into()
    }
}

// --------------------------------------------------------------------------------------
// The WASM implementation of the above that cannot use proc_macro2 and related features,
// the only difference should be in error handling.
// --------------------------------------------------------------------------------------
#[cfg(target_family = "wasm")]
#[proc_macro_attribute]
pub fn input_system(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_fn = match parse::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => panic!(),
    };

    let name = item_fn.sig.ident;

    let inputs: Vec<_> = item_fn
        .sig
        .inputs
        .iter()
        .map(|input| match input {
            FnArg::Receiver(_) => panic!(),
            FnArg::Typed(arg) => {
                let arg_name = match arg.pat.as_ref() {
                    syn::Pat::Ident(ident) => &ident.ident,
                    _ => panic!(),
                };
                let arg_type = match arg.ty.as_ref() {
                    syn::Type::Reference(reference) => match &reference.elem.as_ref() {
                        syn::Type::Path(path) => {
                            path.path.segments.last().unwrap_or_else(|| panic!())
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                };
                (arg_name, arg_type)
            }
        })
        .collect();

    let types = inputs.iter().map(|(_, t)| t);
    let names = inputs.iter().map(|(n, _)| n);

    let body = item_fn.block;

    let output = quote! {

        fn #name(mut query: bevy::prelude::Query<(&mut bevy_utility_ai::AIMeta #(, &#types)*)>) {
            let key = #name as usize;
            for (mut ai_meta #(, #names)*) in query.iter_mut() {
                let score = #body;
                let mut entry = ai_meta.input_scores.entry(key).or_insert(f32::NEG_INFINITY);
                *entry = score;
            }
        }
    };

    output.into()
}

#[cfg(test)]
mod tests {
    use crate::inner::input_system_core;
    use quote::quote;

    #[test]
    fn first() {
        let before = quote! {
            fn utility_input_low(some_data: &SomeData) -> f32 {
                some_data.val
            }
        };

        input_system_core(quote!(), before.into());
    }
}
