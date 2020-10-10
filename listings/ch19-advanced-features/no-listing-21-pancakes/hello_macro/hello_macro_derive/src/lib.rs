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

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("你好，巨集，我叫作{}！", stringify!(#name));
            }
        }
    };
    gen.into()
}
