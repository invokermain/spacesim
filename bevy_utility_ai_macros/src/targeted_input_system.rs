use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
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
    let quoted_name = format!("{}", name);

    if item_fn.sig.inputs.len() > 2 {
        return Err(Error::new_spanned(
            item_fn.sig.inputs.into_token_stream(),
            "Expected two inputs".to_string(),
        ));
    }

    let parsed_inputs_result: Result<Vec<ParsedInput>, Error> = item_fn
        .sig
        .inputs
        .iter()
        .map(parse_input)
        .collect::<Vec<Result<ParsedInput, Error>>>()
        .into_iter()
        .collect();

    let parsed_inputs = parsed_inputs_result?;
    let mut subject_input: Option<&ParsedInput> = None;
    let mut target_input: Option<&ParsedInput> = None;

    for (idx, input) in parsed_inputs.iter().enumerate() {
        match input.ident.to_string().as_str() {
            "subject" => {
                if subject_input.is_some() {
                    return Err(Error::new_spanned(
                        item_fn.sig.inputs[idx].clone().into_token_stream(),
                        "There already exists an input named 'subject'".to_string(),
                    ));
                }
                subject_input = Some(input);
            }
            "target" => {
                if target_input.is_some() {
                    return Err(Error::new_spanned(
                        item_fn.sig.inputs[idx].clone().into_token_stream(),
                        "There already exists an input named 'target'".to_string(),
                    ));
                }
                target_input = Some(input);
            }
            _ => {
                return Err(Error::new_spanned(
                    item_fn.sig.inputs[idx].clone().into_token_stream(),
                    "Function can only have two inputs parameters, one named \
                    'subject' (optional), and one named 'target' (required)"
                        .to_string(),
                ))
            }
        }
    }

    if target_input.is_none() {
        return Err(Error::new_spanned(
            item_fn.sig.inputs.into_token_stream(),
            "Function must have an input named 'target'".to_string(),
        ));
    }

    let ParsedInput {
        ident: target_ident,
        arg_names: target_arg_names,
        arg_types: target_arg_types,
    } = target_input.unwrap();

    let mut subject_ident = None;
    let mut subject_arg_names = Vec::new();
    let mut subject_arg_types = Vec::new();

    if let Some(subject_input) = subject_input {
        subject_ident = Some(subject_input.ident.clone());
        subject_arg_names = subject_input.arg_names.clone();
        subject_arg_types = subject_input.arg_types.clone();
    }

    let subject_data_line = match subject_ident {
        None => TokenStream2::new(),
        Some(ident) => quote! { let #ident = (#(#subject_arg_names, )*); },
    };

    let body = item_fn.block;

    let output = quote! {
        fn #name(
            mut q_subject: bevy::prelude::Query<(bevy::prelude::Entity, &mut bevy_utility_ai::AIMeta #(, &#subject_arg_types)*)>,
            q_target: bevy::prelude::Query<(bevy::prelude::Entity #(, &#target_arg_types)*)>,
            res_ai_definitions: bevy::prelude::Res<bevy_utility_ai::AIDefinitions>,
            archetypes: &bevy::ecs::archetype::Archetypes,
            entities: &bevy::ecs::entity::Entities,
            components: &bevy::ecs::component::Components,
        ) {
            let _span = bevy::prelude::debug_span!("Calculating Targeted Input", input = #quoted_name).entered();
            let key = #name as usize;

            for (subject_entity_id, mut ai_meta #(, #subject_arg_names)*) in q_subject.iter_mut() {
                let _span = bevy::prelude::debug_span!("", entity = subject_entity_id.index()).entered();

                let ai_definition = res_ai_definitions.map.get(&ai_meta.ai_definition).unwrap();
                if !ai_definition.requires_targeted_input(&key) {
                    bevy::prelude::debug!("skipped calculating inputs for this entity");
                    continue;
                };
                let target_filter = &ai_definition
                    .get_targeted_input_requirements(&key)
                    .target_filter;

                let score_map = ai_meta
                    .targeted_input_scores
                    .entry(key)
                    .or_insert(bevy::utils::HashMap::new());

                #subject_data_line

                for (entity_id #(, #target_arg_names)*) in q_target.iter() {
                    let _span = bevy::prelude::debug_span!("", target_entity = entity_id.index()).entered();

                    let matches_filters = {
                        match target_filter {
                            bevy_utility_ai::FilterDefinition::Any => true,
                            bevy_utility_ai::FilterDefinition::Filtered(filter_component_sets) => {
                                let archetype = archetypes
                                    .get(entities.get(entity_id).unwrap().archetype_id)
                                    .unwrap();
                                filter_component_sets.iter().any(|component_set| {
                                    component_set.iter().all(|component_filter| {
                                        match components.get_id(component_filter.component_type_id()) {
                                            Some(component_id) => match component_filter {
                                                bevy_utility_ai::decisions::Filter::Inclusive(_) => archetype.contains(component_id),
                                                bevy_utility_ai::decisions::Filter::Exclusive(_) => !archetype.contains(component_id),
                                            },
                                            None => match component_filter {
                                                bevy_utility_ai::decisions::Filter::Inclusive(_) => false,
                                                bevy_utility_ai::decisions::Filter::Exclusive(_) => true,
                                            },
                                        }
                                    })
                                })
                            }
                        }
                    };

                    if !matches_filters || entity_id == subject_entity_id {
                        continue;
                    }

                    let #target_ident = (#(#target_arg_names, )*);
                    let score =  #body;
                    let entry = score_map.entry(entity_id).or_insert(f32::NEG_INFINITY);
                    *entry = score;
                    bevy::prelude::debug!("score {:.2}", score);
                }
            }
        }
    };

    Ok(output.into())
}

struct ParsedInput {
    ident: Ident,
    arg_names: Vec<Ident>,
    arg_types: Vec<PathSegment>,
}

fn parse_input(input: &FnArg) -> Result<ParsedInput, Error> {
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

    Ok(ParsedInput {
        ident: input_ident,
        arg_names,
        arg_types,
    })
}
