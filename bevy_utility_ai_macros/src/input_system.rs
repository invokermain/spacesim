use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Error, FnArg, ItemFn};

pub(crate) fn input_system(_: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
    let item_fn = match syn::parse::<ItemFn>(input) {
        Ok(ast) => ast,
        Err(err) => return Err(err),
    };

    let name = item_fn.sig.ident;

    let mut arg_names = Vec::new();
    let mut arg_types = Vec::new();

    for input in item_fn.sig.inputs {
        match input {
            FnArg::Receiver(_) => panic!("Input function cannot have self"),
            FnArg::Typed(arg) => {
                arg_names.push(match arg.pat.as_ref() {
                    syn::Pat::Ident(ident) => ident.ident.clone(),
                    _ => {
                        return Err(Error::new_spanned(
                            arg.pat.into_token_stream(),
                            "Expected an Identity".to_string(),
                        ));
                    }
                });
                arg_types.push(match arg.ty.as_ref() {
                    syn::Type::Reference(reference) => match &reference.elem.as_ref() {
                        syn::Type::Path(path) => path
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
                    },
                    _ => {
                        return Err(Error::new_spanned(
                            arg.ty.into_token_stream(),
                            "Expected the parameter type to be a reference".to_string(),
                        ));
                    }
                });
            }
        };
    }

    let body = item_fn.block;

    // TODO: this needs to check against the AIDefinitions resource if the given entity requires
    //   the input. Also update WASM macro.
    let output = quote! {
        fn #name(mut query: bevy::prelude::Query<(&mut bevy_utility_ai::AIMeta #(, &#arg_types)*)>) {
            let key = #name as usize;
            for (mut ai_meta #(, #arg_names)*) in query.iter_mut() {
                let score = #body;
                let mut entry = ai_meta.input_scores.entry(key).or_insert(f32::NEG_INFINITY);
                *entry = score;
            }
        }
    };

    Ok(output.into())
}
