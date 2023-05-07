use proc_macro2::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse2, parse_quote, ItemFn};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn input_system(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input_system_core(args.into(), input.into()).into()
}

fn input_system_core(_args: TokenStream, input: TokenStream) -> TokenStream {
    let old_item_fn = match parse2::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };
    let new_item_fn = transform_fn(old_item_fn);
    println!("after: {}", quote!(#new_item_fn).to_string());
    quote!(#new_item_fn)
}

fn transform_fn(item_fn: ItemFn) -> ItemFn {
    println!("input syntax: {:#?}", item_fn);

    let name = item_fn.sig.ident;
    let body = item_fn.block;

    parse_quote! {
        fn #name() {
            let key = utility_input_system as usize;
            for (mut ai_meta, t) in query.iter_mut() {
                let score = #body;
                ai_meta.input_scores.entry(key).and_modify(|v| *v = score);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let before = quote! {
            fn utility_input_low(some_data: &SomeData) -> f32 {
                some_data.val
            }
        };

        input_system_core(quote!(), before);
    }
}
