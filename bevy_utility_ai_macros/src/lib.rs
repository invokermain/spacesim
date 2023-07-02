pub(crate) mod common;
mod input_system;
mod targeted_input_system;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn input_system(args: TokenStream, input: TokenStream) -> TokenStream {
    match input_system::input_system(args, input) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn targeted_input_system(args: TokenStream, input: TokenStream) -> TokenStream {
    match targeted_input_system::targeted_input_system(args, input) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}
