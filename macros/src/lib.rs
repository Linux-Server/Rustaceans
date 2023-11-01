use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(Profile)]
pub fn deriving(input:TokenStream)->TokenStream{
    let ast=parse_macro_input!(input as DeriveInput);
    println!("AST : {:#?}", ast.ident);
    let name=ast.ident;
    
    let implement=quote!(
        impl Profile for #name{
             fn name(&self){
                println!("My name is {}",stringify!(#name));
            }
        }
    );
    implement.into()
}

#[proc_macro_attribute]
pub fn my_attribute(input:TokenStream,metadta:TokenStream)->TokenStream{
    TokenStream::from(quote!(
        fn new(){
            println!("its new function");
        }
    ))
}