extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;  //将syn包产生的数据结构重新转换为Rust代码
use syn;  //将Rust代码从字符串转换为可供进一步操作的数据结构体

#[proc_macro_derive(HelloMacro)]  //当用户在某个类型上标注#[derive(HelloMacro)]时，hello_macro_derive会被自动调用
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    //将Rust代码转换为我们能够进行处理的语法树
    let ast = syn::parse(input).unwrap();  //接收TokenStream作为输入，DeriveInput结构体作为结果
    //构造对应的trait实现
    impl_hello_macro(&ast)
}
fn impl_hello_macro(ast: &stn::DeriveInput) -> TokenStream {
    let name = ast.ident;  //"Pancakes"字段
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro(){
                //stringify!接收一个Rust表达式，在编译时将其转换为字符串字面量
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()  //将quote!宏执行结果转换换位TokenStream类型
}