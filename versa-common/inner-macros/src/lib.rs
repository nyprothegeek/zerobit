use proc_macro::TokenStream;

//-------------------------------------------------------------------------------------------------
// Macros
//-------------------------------------------------------------------------------------------------

#[proc_macro_attribute]
pub fn describe(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    todo!("describe")
}

#[proc_macro_attribute]
pub fn tool(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    todo!("tool")
}
