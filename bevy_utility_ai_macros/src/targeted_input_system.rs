use crate::common::{parse_input, parse_tuple_input, ParsedInput, ParsedTupleInput, SigType};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Error, FnArg, ItemFn};

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

    if item_fn.sig.inputs.len() == 0 {
        return Err(Error::new_spanned(
            item_fn.sig.inputs.into_token_stream(),
            "Expected at least one input".to_string(),
        ));
    }

    let mut subject_input: Option<ParsedTupleInput> = None;
    let mut target_input: Option<ParsedTupleInput> = None;
    let mut extra_inputs: Vec<ParsedInput> = Vec::new();

    for input in &item_fn.sig.inputs {
        match input {
            FnArg::Receiver(_) => {
                return Err(Error::new_spanned(
                    input.into_token_stream(),
                    "Input cannot have self".to_string(),
                ))
            }
            FnArg::Typed(arg) => {
                let input_ident = match arg.pat.as_ref() {
                    syn::Pat::Ident(ident) => ident.ident.clone(),
                    _ => {
                        return Err(Error::new_spanned(
                            arg.pat.clone().into_token_stream(),
                            "Expected an Identity".to_string(),
                        ));
                    }
                };
                match input_ident.to_string().as_str() {
                    "subject" => {
                        if subject_input.is_some() {
                            return Err(Error::new_spanned(
                                input.clone().into_token_stream(),
                                "There already exists an input named 'subject'".to_string(),
                            ));
                        }
                        subject_input = Some(parse_tuple_input(input)?);
                    }
                    "target" => {
                        if target_input.is_some() {
                            return Err(Error::new_spanned(
                                input.clone().into_token_stream(),
                                "There already exists an input named 'target'".to_string(),
                            ));
                        }
                        target_input = Some(parse_tuple_input(input)?);
                    }
                    _ => {
                        let parsed_input = parse_input(input)?;
                        match parsed_input.sig_type {
                            SigType::Component => {
                                return Err(Error::new_spanned(
                                    input.clone().into_token_stream(),
                                    "This extra input is not valid, expected one of Res, ResMut".to_string(),
                                ));
                            }
                            SigType::Extra => {}
                        };
                        extra_inputs.push(parsed_input);
                    }
                }
            }
        }
    }

    if target_input.is_none() {
        return Err(Error::new_spanned(
            item_fn.sig.inputs.into_token_stream(),
            "Function must have an input named 'target'".to_string(),
        ));
    }

    let ParsedTupleInput {
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

    let extra_args: Vec<proc_macro2::TokenStream> = extra_inputs
        .iter()
        .map(|ParsedInput { ident, tokens, .. }| quote! { #ident: bevy::prelude::#tokens })
        .collect();

    let output = quote! {
        fn #name(
            mut q_subject: bevy::prelude::Query<(bevy::prelude::Entity, &mut bevy_utility_ai::AIMeta #(, &#subject_arg_types)*)>,
            q_target: bevy::prelude::Query<(bevy::prelude::Entity #(, &#target_arg_types)*)>,
            res_ai_definitions: bevy::prelude::Res<bevy_utility_ai::AIDefinitions>,
            archetypes: &bevy::ecs::archetype::Archetypes,
            entities: &bevy::ecs::entity::Entities,
            components: &bevy::ecs::component::Components
            #(, #extra_args)*
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
