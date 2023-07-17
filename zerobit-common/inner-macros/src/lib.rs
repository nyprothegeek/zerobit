use proc_macro::TokenStream;

//-------------------------------------------------------------------------------------------------
// Macros
//-------------------------------------------------------------------------------------------------

#[proc_macro_attribute]
pub fn describe(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    todo!("describe")
}
