#![feature(proc_macro_diagnostic)]

use proc_macro2::Span;
use quote::{quote, ToTokens};
use subcommand::format_subcommand;
use syn::{parse_macro_input, FnArg, ItemFn, Lifetime, ReturnType, Type};

extern crate proc_macro;

mod command;
use command::CommandInput;
mod subcommand;
use subcommand::*;


#[proc_macro]
pub fn command(input_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = parse_macro_input!(input_stream as CommandInput);

    output.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn permissions_check(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let return_ty = match func.sig.output {
        ReturnType::Type(_, t) => t,
        _ => unimplemented!(),
    };
    let name = func.sig.ident;
    let block = func.block;
    let vis = func.vis;
    let attrs = func.attrs;
    let input = func.sig.inputs.iter().map(|arg| {
        if let FnArg::Typed(t) = arg {
            let mut t = t.clone();
            let ty = t.ty.clone();

            t.ty = if let Type::Reference(mut r) = *ty {
                r.lifetime = Some(Lifetime::new("'fut", Span::call_site()));
                Box::new(Type::Reference(r))
            } else {
                ty
            };
            FnArg::Typed(t)
        } else {
            arg.clone()
        }
    });


    let token_stream = quote! {
        #(#attrs)*
        #vis fn #name<'fut>(#(#input),*) -> ::serenity::futures::future::BoxFuture<'fut, #return_ty> {
            use ::serenity::futures::future::FutureExt;
            async move {
                #block
            }
            .boxed()
        }
    };

    token_stream.into()
}

#[proc_macro_attribute]
pub fn subcommand(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(attr as SubCommandArgs);
    let func = parse_macro_input!(item as SubCommandFunc);
    format_subcommand(func, args)
}
