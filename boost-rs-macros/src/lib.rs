use proc_macro::TokenStream;

mod bench;
mod hello;

#[proc_macro_derive(HelloMacro)]
pub fn some_name(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast = syn::parse(input).unwrap();

    // 构建特征实现代码
    hello::impl_hello_macro(&ast)
}

#[proc_macro_attribute]
pub fn elapsed(args: TokenStream, func: TokenStream) -> TokenStream {
    bench::elapsed(args, func)
}
