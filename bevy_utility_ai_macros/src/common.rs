use quote::{format_ident, ToTokens};
use syn::{Error, FnArg, Ident, PathSegment, Type};

#[derive(Eq, PartialEq)]
pub(crate) enum SigType {
    Component,
    Extra,
    Entity,
}

pub(crate) struct ParsedInput {
    pub(crate) sig_type: SigType,
    pub(crate) ident: proc_macro2::Ident,
    pub(crate) tokens: proc_macro2::TokenStream,
}

pub(crate) const UNEXPECTED_TYPE_ERR: &str =
    "Expected the parameter type to be a reference, Res or ResMut";
pub(crate) const REQUIRES_REFERENCE_ERR: &str = "Only Entity cannot be borrowed here";
pub(crate) const ACCEPTED_EXTRA_SIGNATURE_TYPES: [&str; 2] = ["Res", "ResMut"];

pub(crate) fn parse_input(input: &FnArg) -> Result<ParsedInput, Error> {
    match input {
        FnArg::Receiver(_) => panic!("Input function cannot have self"),
        FnArg::Typed(arg) => {
            let ident = match arg.pat.as_ref() {
                syn::Pat::Ident(ident) => ident.ident.clone(),
                _ => {
                    return Err(Error::new_spanned(
                        arg.pat.clone().into_token_stream(),
                        "Expected an Identity".to_string(),
                    ));
                }
            };
            let (sig_type, tokens) = match arg.ty.as_ref() {
                Type::Reference(reference) => match &reference.elem.as_ref() {
                    Type::Path(path) => (
                        SigType::Component,
                        path.path
                            .segments
                            .last()
                            .unwrap_or_else(|| panic!("What? {:?}", path.path))
                            .to_token_stream(),
                    ),
                    _ => {
                        return Err(Error::new_spanned(
                            reference.elem.clone().into_token_stream(),
                            "Expected a Component".to_string(),
                        ));
                    }
                },
                Type::Path(path) => {
                    if let Some(first_segment) = path.path.segments.first() {
                        let arg_type_str = first_segment.ident.to_string();
                        if ACCEPTED_EXTRA_SIGNATURE_TYPES.contains(&arg_type_str.as_str()) {
                            (SigType::Extra, path.to_token_stream())
                        } else if &arg_type_str == "Entity" {
                            (SigType::Entity, path.to_token_stream())
                        } else {
                            return Err(Error::new_spanned(
                                arg.ty.clone().into_token_stream(),
                                UNEXPECTED_TYPE_ERR.to_string(),
                            ));
                        }
                    } else {
                        return Err(Error::new_spanned(
                            arg.ty.clone().into_token_stream(),
                            UNEXPECTED_TYPE_ERR.to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(Error::new_spanned(
                        arg.ty.clone().into_token_stream(),
                        UNEXPECTED_TYPE_ERR.to_string(),
                    ));
                }
            };
            Ok(ParsedInput {
                ident,
                sig_type,
                tokens,
            })
        }
    }
}

pub(crate) struct ParsedTupleInput {
    pub(crate) ident: Ident,
    pub(crate) arg_names: Vec<Ident>,
    pub(crate) arg_types: Vec<PathSegment>,
}

pub(crate) fn parse_tuple_input(input: &FnArg) -> Result<ParsedTupleInput, Error> {
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
                            Type::Path(path) => {
                                if let Some(first_segment) = path.path.segments.first() {
                                    if first_segment.ident.to_string().as_str() == "Entity" {
                                        arg_types.push(first_segment.clone());
                                    } else {
                                        return Err(Error::new_spanned(
                                            arg.ty.clone().into_token_stream(),
                                            REQUIRES_REFERENCE_ERR.to_string(),
                                        ));
                                    }
                                } else {
                                    return Err(Error::new_spanned(
                                        arg.ty.clone().into_token_stream(),
                                        REQUIRES_REFERENCE_ERR.to_string(),
                                    ));
                                }
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

    Ok(ParsedTupleInput {
        ident: input_ident,
        arg_names,
        arg_types,
    })
}
