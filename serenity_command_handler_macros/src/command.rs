use proc_macro2::Literal;
use quote::{format_ident, quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{
    braced,
    bracketed,
    ext::IdentExt,
    parse::Parse,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Bracket, Comma},
    Ident,
    Token,
};


mod kw {
    use syn::custom_keyword;

    custom_keyword!(required);
    custom_keyword!(optional);
    custom_keyword!(SubCommand);
    custom_keyword!(SubCommandGroup);
    custom_keyword!(String);
    custom_keyword!(Integer);
    custom_keyword!(Boolean);
    custom_keyword!(User);
    custom_keyword!(Channel);
    custom_keyword!(Role);
}

pub struct CommandInput {
    name: Ident,
    description: Literal,
    tree: Option<Punctuated<Argument, Token![,]>>,
    func: Option<Ident>,
}

impl Parse for CommandInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.call(Ident::parse_any)?;
        input.parse::<Token![,]>()?;
        let description = input.parse()?;
        input.parse::<Token![,]>()?;

        let func = if input.peek(Bracket) {
            None
        } else {
            let a = Some(input.parse()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            a
        };

        let tree = if input.peek(Bracket) {
            let content;
            bracketed!(content in input);

            let raw_children = content.parse_terminated(Argument::parse)?;

            let mut optional = false;
            for child in &raw_children {
                match child.required {
                    Required::Required(r) =>
                        if optional {
                            r.span()
                                .unwrap()
                                .error("Cannot have required argument after optional")
                                .emit();
                        },
                    Required::Optional(_) => {
                        optional = true;
                    }
                }
            }

            Some(raw_children)
        } else {
            None
        };

        Ok(CommandInput {
            name,
            description,
            tree,
            func,
        })
    }
}

impl ToTokens for CommandInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let description = &self.description;
        let func = match &self.func {
            Some(f) => quote! {Some(#f)},
            None => quote! {None},
        };
        let tree = match &self.tree {
            Some(t) => {
                let t = t.iter();
                quote! {Some(vec![#(#t),*])}
            }
            None => quote! {None},
        };
        let struct_name = format_ident!("{}_COMMAND", name.to_string().to_uppercase());
        let cmd_name = format!("{}", name);
        tokens.append_all(quote! {
            use ::serenity_command_handler::commands::{Command, CommandArguments, ArgumentChoice, CommandArgumentsTree};
            use ::serenity_command_handler::framework::CommandInit;
            use std::iter::FromIterator;
            pub struct #struct_name;
            impl CommandInit for #struct_name {
                fn command_init() -> Command {
                    Command {
                        name: #cmd_name,
                        description: #description,
                        arguments_tree: CommandArgumentsTree {
                            children: #tree,
                            func: #func
                        }
                    }
                }
            }
        });
    }
}

struct Argument {
    required: Required,
    ty: ArgType,
    name: Ident,
    func: Option<Ident>,
    description: Literal,
    options: Option<ArgumentOption>,
    children: Option<Punctuated<Argument, Comma>>,
}

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let required = &self.required;
        let ty = &self.ty;
        let name = &self.name.to_string();
        let options = match &self.options {
            Some(o) => quote! {Some(#o)},
            None => quote! {None},
        };
        let description = &self.description;
        let children = match &self.children {
            Some(c) => {
                let c = c.iter();
                quote! {Some(vec![#(#c),*])}
            }
            None => quote! {None},
        };

        let func = match &self.func {
            Some(i) => quote! {Some(#i)},
            None => quote! {None},
        };

        let span = self.name.span();

        tokens.append_all(match ty {
            ArgType::SubCommand(_) | ArgType::SubCommandGroup(_) => quote_spanned! {span=>
                #ty {
                    name: #name,
                    description: #description,
                    required: #required,
                    options: #children,
                    func: #func
                }
            },
            ArgType::String(_) | ArgType::Integer(_) => quote_spanned! {span=>
                #ty {
                    name: #name,
                    description: #description,
                    required: #required,
                    choices: #options,
                }
            },
            _ => quote_spanned! {span=>
                #ty {
                    name: #name,
                    description: #description,
                    required: #required,
                }
            },
        })
    }
}

impl Parse for Argument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let required = input.parse::<Required>()?;
        let ty = input.parse::<ArgType>()?;
        let name = input.call(Ident::parse_any)?;

        let mut func = None;
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            func = Some(input.parse()?);
        }

        input.parse::<Token![|]>()?;
        let description = input.parse()?;

        let mut options = None;
        if input.peek(Brace) {
            options = Some(input.parse()?);
        }

        let mut children = None;
        if input.peek(Bracket) {
            let content;
            bracketed!(content in input);
            let raw_children = content.parse_terminated(Argument::parse)?;

            let mut optional = false;

            for child in &raw_children {
                match child.required {
                    Required::Required(r) =>
                        if optional {
                            r.span()
                                .unwrap()
                                .error("Cannot have required argument after optional")
                                .emit();
                        },
                    Required::Optional(_) => {
                        optional = true;
                    }
                }
            }

            children = Some(raw_children);
        }

        Ok(Argument {
            required,
            ty,
            name,
            options,
            description,
            children,
            func,
        })
    }
}

struct ArgumentOption {
    options: Punctuated<ChoiceMap, Token![,]>,
}

impl Parse for ArgumentOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        Ok(ArgumentOption {
            options: content.parse_terminated(ChoiceMap::parse)?,
        })
    }
}

impl ToTokens for ArgumentOption {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let options = self.options.iter().to_owned();
        tokens.append_all(quote! {vec![#(#options),*]})
    }
}

struct ChoiceMap {
    key: Literal,
    value: Literal,
}

impl ToTokens for ChoiceMap {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let key = &self.key;
        let value = &self.value;
        tokens.append_all(quote! {ArgumentChoice{ name: #key, value: #value}})
    }
}

impl Parse for ChoiceMap {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![:]>()?;
        let value = input.parse()?;
        Ok(ChoiceMap { key, value })
    }
}


enum Required {
    Required(kw::required),
    Optional(kw::optional),
}

impl ToTokens for Required {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match self {
            Required::Optional(_) => quote! {false},
            Required::Required(_) => quote! {true},
        });
    }
}

impl Parse for Required {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::required) {
            Ok(Required::Required(input.parse()?))
        } else if lookahead.peek(kw::optional) {
            Ok(Required::Optional(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

enum ArgType {
    SubCommand(kw::SubCommand),
    SubCommandGroup(kw::SubCommandGroup),
    String(kw::String),
    Integer(kw::Integer),
    Boolean(kw::Boolean),
    User(kw::User),
    Channel(kw::Channel),
    Role(kw::Role),
}

impl ToTokens for ArgType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match self {
            ArgType::SubCommand(_) => quote! {CommandArguments::SubCommand},
            ArgType::SubCommandGroup(_) => quote! {CommandArguments::SubCommandGroup},
            ArgType::String(_) => quote! {CommandArguments::String},
            ArgType::Integer(_) => quote! {CommandArguments::Integer},
            ArgType::Boolean(_) => quote! {CommandArguments::Boolean},
            ArgType::User(_) => quote! {CommandArguments::User},
            ArgType::Channel(_) => quote! {CommandArguments::Channel},
            ArgType::Role(_) => quote! {CommandArguments::Role},
        })
    }
}

macro_rules! arg_type_parser {
    ($lookahead: ident, $input: ident, $($name: ident),*) => {
        $(if $lookahead.peek(kw::$name) {
            Ok(ArgType::$name($input.parse()?))
        }else)* {
            Err($lookahead.error())
        }
    };
}

impl Parse for ArgType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        arg_type_parser!(
            lookahead,
            input,
            SubCommand,
            SubCommandGroup,
            String,
            Integer,
            Boolean,
            User,
            Channel,
            Role
        )
    }
}
