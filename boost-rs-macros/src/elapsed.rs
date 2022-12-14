use proc_macro::TokenStream;
use std::fmt::format;

use quote::quote;
use syn::ItemFn;
use syn::parse_macro_input;

pub(crate) fn elapsed(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis; // like pub
    let func_block = &func.block; // { some statement or expression here }

    let func_decl = func.sig;
    let func_name = &func_decl.ident; // function name
    let func_name_str = format!("\"{}\"", func_name);
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let caller = quote! {
        // rebuild the function, add a func named is_expired to check user login session expire or not.
        #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
            use std::time;
            let start = time::Instant::now();
            let func_name = String::from(#func_name_str);
            #func_block
            println!("Run in {} cost time: {:?}", func_name, start.elapsed());
        }
    };

    caller.into()
}
