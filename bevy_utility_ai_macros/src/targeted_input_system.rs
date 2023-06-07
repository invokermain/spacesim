use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Error, FnArg, Ident, ItemFn, PathSegment, Type};

pub(crate) fn targeted_input_system(
    _: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, Error> {
    let item_fn = match syn::parse::<ItemFn>(input) {
        Ok(ast) => ast,
        Err(err) => return Err(err),
    };

    let name = item_fn.sig.ident;

    if item_fn.sig.inputs.len() != 2 {
        return Err(Error::new_spanned(
            item_fn.sig.inputs.into_token_stream(),
            "Expected two inputs".to_string(),
        ));
    }

    let subject_input = &item_fn.sig.inputs[0];
    let target_input = &item_fn.sig.inputs[1];

    let (subject_ident, subject_arg_names, subject_arg_types) = parse_input(subject_input)?;
    let (target_ident, target_arg_names, target_arg_types) = parse_input(target_input)?;

    let body = item_fn.block;

    // TODO: this needs to check against the AIDefinitions resource if the given entity requires
    //   the input. Also update WASM macro.
    let output = quote! {
        fn #name(
            mut q_subject: bevy::prelude::Query<(bevy::prelude::Entity, &mut bevy_utility_ai::AIMeta #(, &#subject_arg_types)*)>,
            q_target: bevy::prelude::Query<(bevy::prelude::Entity #(, &#target_arg_types)*)>
        ) {
            let key = #name as usize;

            for (subject_entity_id, mut ai_meta #(, #subject_arg_names)*) in q_subject.iter_mut() {
                let score_map = ai_meta
                    .targeted_input_scores
                    .entry(key)
                    .or_insert(bevy::utils::HashMap::new());
                let #subject_ident = (#(#subject_arg_names, )*);
                for (entity_id #(, #target_arg_names)*) in q_target.iter() {
                    if entity_id == subject_entity_id {
                        continue;
                    }
                    let #target_ident = (#(#target_arg_names, )*);
                    let score =  #body;
                    let entry = score_map.entry(entity_id).or_insert(f32::NEG_INFINITY);
                    *entry = score;
                }
            }
        }
    };

    Ok(output.into())
}

fn parse_input(input: &FnArg) -> Result<(Ident, Vec<Ident>, Vec<PathSegment>), Error> {
    let input_ident: Ident;
    let mut arg_types = Vec::new();

    match input {
        FnArg::Receiver(_) => panic!("Input function cannot have self"),
        FnArg::Typed(arg) => {
            match arg.pat.as_ref() {
                syn::Pat::Ident(ident) => {
                    input_ident = ident.ident.clone();
                }
                _ => {
                    return Err(Error::new_spanned(
                        arg.pat.clone().into_token_stream(),
                        "Expected an Identity".to_string(),
                    ));
                }
            };
            match arg.ty.as_ref() {
                Type::Tuple(tuple) => {
                    for tuple_elem in &tuple.elems {
                        match tuple_elem {
                            Type::Reference(reference) => {
                                arg_types.push(match &reference.elem.as_ref() {
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
                        arg.ty.clone().into_token_stream(),
                        "Expected the parameter type to be a tuple".to_string(),
                    ));
                }
            };
        }
    };
    let arg_names: Vec<Ident> = arg_types
        .iter()
        .enumerate()
        .map(|(idx, _)| format_ident!("p{idx}"))
        .collect();

    Ok((input_ident, arg_names, arg_types))
}
