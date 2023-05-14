use quote::quote;
use syn::{parse, FnArg, ItemFn};

#[proc_macro_attribute]
pub fn input_system(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_fn = match parse::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(_) => panic!(), // return error.to_compile_error(),
    };
    // println!("input syntax: {:#?}", item_fn);

    let name = item_fn.sig.ident;

    let inputs: Vec<_> = item_fn
        .sig
        .inputs
        .iter()
        .map(|input| match input {
            FnArg::Receiver(_) => panic!(), //abort!(input, "Input function cannot have self"),
            FnArg::Typed(arg) => {
                let arg_name = match arg.pat.as_ref() {
                    syn::Pat::Ident(ident) => &ident.ident,
                    _ => panic!(), // abort!(arg, "Expected Ident?"),
                };
                let arg_type = match arg.ty.as_ref() {
                    syn::Type::Reference(reference) => match &reference.elem.as_ref() {
                        syn::Type::Path(path) => {
                            path.path.segments.last().unwrap_or_else(|| panic!())
                        } //abort!(path.path, "What?")),
                        _ => panic!(), // abort!(reference.elem, "Expected a Component"),
                    },
                    _ => panic!(), // abort!(arg.ty, "Expected the parameter type to be a reference"),
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
                ai_meta.input_scores.entry(key).and_modify(|v| *v = score);
            }
        }
    };

    // println!("after: {}", output.to_string());

    output.into()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn first() {
//         let before = quote! {
//             fn utility_input_low(some_data: &SomeData) -> f32 {
//                 some_data.val
//             }
//         };

//         input_system_core(quote!(), before);
//     }
// }
