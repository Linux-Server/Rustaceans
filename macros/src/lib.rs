#![allow(dead_code)]
#![allow(unused_variables)]


use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(Profile)]
pub fn deriving(input:TokenStream)->TokenStream{
    println!("Token Tree : {:#?}", input); // token tree
    let ast=syn::parse_macro_input!(input as syn::DeriveInput); // ast
    println!("AST : {:#?}", ast); // AST

    let name=ast.ident;

    let implement=quote!(
        impl Profile for #name{ // Person
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