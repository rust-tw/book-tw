extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 建構 Rust 程式碼的語法樹呈現
    // 讓我們可以進行操作
    let ast = syn::parse(input).unwrap();

    // 建構特徵實作
    impl_hello_macro(&ast)
}
