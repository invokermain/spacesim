use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Error, FnArg, Ident, ItemFn, Type};

pub(crate) fn targeted_input_system(
    _: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, Error> {
    let item_fn = match syn::parse::<ItemFn>(input) {
        Ok(ast) => ast,
        Err(err) => return Err(err),
    };

    let name = item_fn.sig.ident;

    if item_fn.sig.inputs.len() != 3 {
        return Err(Error::new_spanned(
            item_fn.sig.inputs.into_token_stream(),
            "Expected two inputs".to_string(),
        ));
    }

    let subject_input = &item_fn.sig.inputs[0];
    let target_input = &item_fn.sig.inputs[2];

    let subject_ident: Ident;
    let subject_arg_types = Vec::new();

    // TODO: finish this macro
    match subject_input {
        FnArg::Receiver(_) => panic!("Input function cannot have self"),
        FnArg::Typed(arg) => {
            match arg.pat.as_ref() {
                syn::Pat::Ident(ident) => {
                    subject_ident = ident.ident.clone();
                }
                _ => {
                    return Err(Error::new_spanned(
                        arg.pat.into_token_stream(),
                        "Expected an Identity".to_string(),
                    ));
                }
            };
            match arg.ty.as_ref() {
                Type::Tuple(tuple) => {
                    for tuple_elem in tuple.elems {
                        match tuple_elem {
                            Type::Reference(reference) => {
                                subject_arg_types.push(match &reference.elem.as_ref() {
                                    Type::Path(path) => path
                                        .path
                                        .segments
                                        .last()
                                        .unwrap_or_else(|| panic!("What? {:?}", path.path))
                                        .clone(),
                                    _ => {
                                        return Err(Error::new_spanned(
                                            reference.elem.clone().into_token_stream(),
                                            "Expected a Component".to_string(),
                                        ));
                                    }
                                })
                            }
                            _ => {
                                return Err(Error::new_spanned(
                                    tuple_elem.into_token_stream(),
                                    "Expected the parameter type to be a reference"
                                        .to_string(),
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::new_spanned(
                        arg.ty.into_token_stream(),
                        "Expected the parameter type to be a tuple".to_string(),
                    ));
                }
            };
        }
    };

    let body = item_fn.block;

    // TODO: this needs to check against the AIDefinitions resource if the given entity requires
    //   the input. Also update WASM macro.
    let output = quote! {
        fn #name(mut query: bevy::prelude::Query<(&mut bevy_utility_ai::AIMeta #(, &#arg_types)*)>) {
            let key = #name as usize;

            for (subject_entity_id, mut ai_meta #(, #arg_names)*) in query.iter_mut() {
                let score = #body;
                let mut entry = ai_meta.input_scores.entry(key).or_insert(f32::NEG_INFINITY);
                *entry = score;
            }
        }
    };

    Ok(output.into())
}
