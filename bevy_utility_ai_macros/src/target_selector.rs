use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Error, FnArg, ItemFn, Pat, Type};

pub(crate) fn target_selector(
    _: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, Error> {
    let item_fn = match syn::parse::<ItemFn>(input) {
        Ok(ast) => ast,
        Err(err) => return Err(err),
    };

    let name = item_fn.sig.ident;
    let body = item_fn.block;

    if item_fn.sig.inputs.len() != 1 {
        return Err(Error::new_spanned(
            item_fn.sig.inputs.into_token_stream(),
            "Expected only one argument".to_string(),
        ));
    }

    let input = match &item_fn.sig.inputs[0] {
        FnArg::Receiver(_) => panic!(),
        FnArg::Typed(typed_input) => typed_input,
    };

    let input_ident = match input.pat.as_ref() {
        Pat::Ident(ident) => ident,
        _ => {
            return Err(Error::new_spanned(
                input,
                "expected an identity here!".to_string(),
            ))
        }
    };

    let input_ty = match input.ty.as_ref() {
        Type::Path(path) => path.to_token_stream(),
        _ => {
            return Err(Error::new_spanned(
                input,
                "expected an type path here!".to_string(),
            ));
        }
    };

    let output = quote! {
        fn #name(
            mut q_subject: bevy::prelude::Query<(bevy::prelude::Entity, &mut bevy_utility_ai::AIMeta)>,
            #input_ident: #input_ty,
            ai_definitions: bevy::prelude::Res<bevy_utility_ai::AIDefinitions>,
        ) {
            let target_selector_key = #name as usize;
            for (entity, mut ai_meta) in q_subject.iter_mut() {
                let targets = #body;


                if let Some(val) = ai_meta.targeted_input_targets.get_mut(&key) {
                    *val = targets;
                } else {
                    ai_meta.targeted_input_targets.insert(key, targets);
                }
            }
        }
    };

    Ok(output.into())
}
