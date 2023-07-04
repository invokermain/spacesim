use crate::common::{parse_input, ParsedInput, SigType};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, ItemFn};

pub(crate) fn input_system(_: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
    let item_fn = match syn::parse::<ItemFn>(input) {
        Ok(ast) => ast,
        Err(err) => return Err(err),
    };

    let name = item_fn.sig.ident;
    let quoted_name = format!("{}", name);

    let parsed_sig: Vec<ParsedInput> = item_fn
        .sig
        .inputs
        .iter()
        .map(parse_input)
        .collect::<Result<Vec<ParsedInput>, Error>>()?;

    let component_arg_types: Vec<proc_macro2::TokenStream> = parsed_sig
        .iter()
        .filter(|sig| sig.sig_type == SigType::Component)
        .map(|sig| sig.tokens.clone())
        .collect();
    let component_arg_idents = parsed_sig
        .iter()
        .filter(|sig| sig.sig_type == SigType::Component)
        .map(|sig| sig.ident.clone());
    let extra_args: Vec<proc_macro2::TokenStream> = parsed_sig
        .iter()
        .filter(|sig| sig.sig_type == SigType::Extra)
        .map(|ParsedInput { ident, tokens, .. }| quote! { #ident: bevy::prelude::#tokens })
        .collect();

    let body = item_fn.block;

    let output = quote! {
        fn #name(
            mut query_input_system: bevy::prelude::Query<(bevy::prelude::Entity, &mut bevy_utility_ai::AIMeta #(, &#component_arg_types)*)>,
            res_ai_definitions: bevy::prelude::Res<bevy_utility_ai::AIDefinitions>
            #(, #extra_args)*
        ) {
            let _span = bevy::prelude::debug_span!("Calculating Input", input = #quoted_name).entered();
            let key = bevy_utility_ai::utils::type_id_of(&#name);

            for (entity, mut ai_meta #(, #component_arg_idents)*) in query_input_system.iter_mut() {
                let _span = bevy::prelude::debug_span!("", entity = entity.index()).entered();

                let ai_definition = &res_ai_definitions.map[&ai_meta.ai_definition];

                if !ai_definition.requires_simple_input(&key) {
                    bevy::prelude::debug!("skipped calculating inputs for this entity");
                    continue;
                };

                let score = #body;
                let mut entry = ai_meta.input_scores.entry(key).or_insert(f32::NEG_INFINITY);
                *entry = score;
                bevy::prelude::debug!("score {:.2}", score);
            }
        }
    };

    Ok(output.into())
}
