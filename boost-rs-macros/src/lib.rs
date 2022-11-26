#![allow(clippy::needless_doctest_main, unused_imports, unused)]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! Macros for use with Boost-rs

use proc_macro::TokenStream;

mod elapsed;
mod hello;

/// A derive macro for testing
#[proc_macro_derive(HelloMacro)]
#[cfg(not(test))]
pub fn hello_macro(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast = syn::parse(input).unwrap();

    // 构建特征实现代码
    hello::impl_hello_macro(&ast)
}

/// A proc macro for calculating the elapsed time of the function
#[proc_macro_attribute]
#[cfg(not(test))]
pub fn elapsed(args: TokenStream, func: TokenStream) -> TokenStream {
    elapsed::elapsed(args, func)
}
