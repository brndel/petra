mod table;
mod quotes;

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quotes::impl_table;

#[proc_macro_derive(Table, attributes(primary, unique, foreign, on_update, on_delete, foreign_link))]
pub fn derive_table(input: TokenStream) -> TokenStream {
  let result = parse(input);
  match result {
    Ok(result) => result,
    Err(error) => to_compile_errors(error),
  }
}

fn parse(input: TokenStream) -> Result<TokenStream, syn::Error> {
  let ast = syn::parse(input)?;

  impl_table(ast)
}

fn to_compile_errors(error: syn::Error) -> TokenStream {
  let errors = error.to_compile_error();

  errors.into()
}