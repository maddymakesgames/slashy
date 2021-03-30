#![feature(prelude_import)]
#![warn(missing_docs)]
//! A command framework for Serenity that allows commands to be registered both as a traditional text command and a discord slash command
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
/// Everything related to parsing and representing arguments.
pub mod argument {
    use std::{collections::HashMap, iter::Peekable, slice::Iter};
    use serenity::model::{
        id::{ChannelId, RoleId, UserId},
        interactions::{ApplicationCommandInteractionDataOption as InteractionOption, Interaction},
    };
    use regex::Regex;
    use crate::{
        commands::{CommandArguments, CommandArgumentsTree, CommandFunction},
        framework::CommandSource,
    };
    /// Represents the argument data sent into commands
    #[allow(missing_docs)]
    pub enum Argument {
        String(String),
        Integer(i32),
        Boolean(bool),
        User(UserId),
        Channel(ChannelId),
        Role(RoleId),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(missing_docs)]
    impl ::core::fmt::Debug for Argument {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Argument::String(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("String");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Argument::Integer(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Integer");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Argument::Boolean(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Boolean");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Argument::User(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("User");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Argument::Channel(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Channel");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Argument::Role(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Role");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(missing_docs)]
    impl ::core::marker::StructuralPartialEq for Argument {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(missing_docs)]
    impl ::core::cmp::PartialEq for Argument {
        #[inline]
        fn eq(&self, other: &Argument) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Argument::String(ref __self_0), &Argument::String(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Argument::Integer(ref __self_0), &Argument::Integer(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Argument::Boolean(ref __self_0), &Argument::Boolean(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Argument::User(ref __self_0), &Argument::User(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Argument::Channel(ref __self_0), &Argument::Channel(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Argument::Role(ref __self_0), &Argument::Role(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Argument) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Argument::String(ref __self_0), &Argument::String(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Argument::Integer(ref __self_0), &Argument::Integer(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Argument::Boolean(ref __self_0), &Argument::Boolean(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Argument::User(ref __self_0), &Argument::User(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Argument::Channel(ref __self_0), &Argument::Channel(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Argument::Role(ref __self_0), &Argument::Role(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    impl Argument {
        fn parse_int(string: &str) -> Result<Self, ()> {
            match string.parse::<i32>() {
                Ok(i) => Ok(Argument::Integer(i)),
                Err(_) => Err(()),
            }
        }
        fn parse_bool(string: &str) -> Result<Self, ()> {
            match string.parse::<bool>() {
                Ok(i) => Ok(Argument::Boolean(i)),
                Err(_) => Err(()),
            }
        }
        fn parse_role_id(string: &str) -> Result<Self, ()> {
            Ok(Argument::Role(RoleId(Self::parse_id_int(string)?)))
        }
        fn parse_user_id(string: &str) -> Result<Self, ()> {
            Ok(Argument::User(UserId(Self::parse_id_int(string)?)))
        }
        fn parse_channel_id(string: &str) -> Result<Self, ()> {
            Ok(Argument::Channel(ChannelId(Self::parse_id_int(string)?)))
        }
        /// Traverses the argument tree of `cmd` and outputs a map of arguments and the function to run
        pub fn parse(
            source: &CommandSource,
            tree: &CommandArgumentsTree,
        ) -> Option<(HashMap<String, Self>, CommandFunction)> {
            match source {
                CommandSource::Interaction(interaction) => {
                    Argument::parse_interaction(interaction, tree)
                }
                CommandSource::Message(message) => Argument::parse_message(&message.content, tree),
                #[cfg(test)]
                CommandSource::Test(str) => Argument::parse_message(str, tree),
            }
        }
        /// Parses [InteractionOptions](InteractionOption) into Argument and gets the function pointer for the node we need to run
        pub fn parse_interaction(
            interaction: &Interaction,
            tree: &CommandArgumentsTree,
        ) -> Option<(HashMap<String, Self>, CommandFunction)> {
            let mut output = HashMap::new();
            let options = Self::get_arguments_from_interaction(interaction);
            if options.len() == 0 || tree.children.is_none() {
                if tree.func.is_some() {
                    Some((output, tree.func.unwrap()))
                } else {
                    None
                }
            } else {
                let nodes = tree.children.as_ref().unwrap();
                let options_iter = options.iter();
                let func: Option<CommandFunction> = Self::parse_interaction_tree(
                    nodes,
                    &mut options_iter.peekable(),
                    &mut output,
                    tree.func,
                );
                match func {
                    Some(f) => Some((output, f)),
                    None => None,
                }
            }
        }
        fn parse_interaction_tree(
            branch: &Vec<CommandArguments>,
            recieved: &mut Peekable<Iter<InteractionOption>>,
            map: &mut HashMap<String, Self>,
            func: Option<CommandFunction>,
        ) -> Option<CommandFunction> {
            let mut curr_arg = recieved.next().unwrap();
            let mut fun = func;
            for arg in branch {
                match arg {
                    CommandArguments::String { name, required, .. } => {
                        if &curr_arg.name == name {
                            match { curr_arg.value.as_ref().unwrap().as_str() } {
                                Some(v) => {
                                    map.insert(name.to_string(), Argument::String(v.to_owned()));
                                    if recieved.peek().is_none() {
                                        break;
                                    }
                                    curr_arg = recieved.next().unwrap();
                                }
                                None => {
                                    if *required {
                                        return None;
                                    }
                                }
                            }
                        }
                    }
                    CommandArguments::Integer { name, required, .. } => {
                        if &curr_arg.name == name {
                            match { curr_arg.value.as_ref().unwrap().as_i64().map(|u| u as i32) } {
                                Some(v) => {
                                    map.insert(name.to_string(), Argument::Integer(v.to_owned()));
                                    if recieved.peek().is_none() {
                                        break;
                                    }
                                    curr_arg = recieved.next().unwrap();
                                }
                                None => {
                                    if *required {
                                        return None;
                                    }
                                }
                            }
                        }
                    }
                    CommandArguments::Boolean { name, required, .. } => {
                        if &curr_arg.name == name {
                            match { curr_arg.value.as_ref().unwrap().as_bool() } {
                                Some(v) => {
                                    map.insert(name.to_string(), Argument::Boolean(v.to_owned()));
                                    if recieved.peek().is_none() {
                                        break;
                                    }
                                    curr_arg = recieved.next().unwrap();
                                }
                                None => {
                                    if *required {
                                        return None;
                                    }
                                }
                            }
                        }
                    }
                    CommandArguments::User { name, required, .. } => {
                        if &curr_arg.name == name {
                            match {
                                curr_arg
                                    .value
                                    .as_ref()
                                    .unwrap()
                                    .as_i64()
                                    .map(|u| UserId(u as u64))
                            } {
                                Some(v) => {
                                    map.insert(name.to_string(), Argument::User(v.to_owned()));
                                    if recieved.peek().is_none() {
                                        break;
                                    }
                                    curr_arg = recieved.next().unwrap();
                                }
                                None => {
                                    if *required {
                                        return None;
                                    }
                                }
                            }
                        }
                    }
                    CommandArguments::Channel { name, required, .. } => {
                        if &curr_arg.name == name {
                            match {
                                curr_arg
                                    .value
                                    .as_ref()
                                    .unwrap()
                                    .as_i64()
                                    .map(|u| ChannelId(u as u64))
                            } {
                                Some(v) => {
                                    map.insert(name.to_string(), Argument::Channel(v.to_owned()));
                                    if recieved.peek().is_none() {
                                        break;
                                    }
                                    curr_arg = recieved.next().unwrap();
                                }
                                None => {
                                    if *required {
                                        return None;
                                    }
                                }
                            }
                        }
                    }
                    CommandArguments::Role { name, required, .. } => {
                        if &curr_arg.name == name {
                            match {
                                curr_arg
                                    .value
                                    .as_ref()
                                    .unwrap()
                                    .as_i64()
                                    .map(|u| RoleId(u as u64))
                            } {
                                Some(v) => {
                                    map.insert(name.to_string(), Argument::Role(v.to_owned()));
                                    if recieved.peek().is_none() {
                                        break;
                                    }
                                    curr_arg = recieved.next().unwrap();
                                }
                                None => {
                                    if *required {
                                        return None;
                                    }
                                }
                            }
                        }
                    }
                    CommandArguments::SubCommand {
                        name,
                        required,
                        func,
                        options,
                        ..
                    } => {
                        if &curr_arg.name == name {
                            fun = func.clone();
                            if let Some(children) = options {
                                fun = Self::parse_interaction_tree(children, recieved, map, fun);
                            }
                        } else {
                            if *required {
                                return None;
                            }
                        }
                    }
                    CommandArguments::SubCommandGroup {
                        name,
                        required,
                        func,
                        options,
                        ..
                    } => {
                        if &curr_arg.name == name {
                            fun = func.clone();
                            if let Some(children) = options {
                                fun = Self::parse_interaction_tree(children, recieved, map, fun);
                            }
                        } else {
                            if *required {
                                return None;
                            }
                        }
                    }
                    #[allow(unreachable_patterns)]
                    _ => {}
                };
            }
            match fun {
                Some(f) => Some(f),
                None => None,
            }
        }
        fn get_arguments_from_interaction(interaction: &Interaction) -> Vec<InteractionOption> {
            let mut output = Vec::new();
            for option in interaction.data.clone().unwrap().options {
                output.extend(Self::traverse_tree(&option))
            }
            output
        }
        fn traverse_tree(interaction: &InteractionOption) -> Vec<InteractionOption> {
            let mut output = Vec::new();
            output.push(interaction.clone());
            for child in interaction.clone().options {
                if child.options.len() > 0 {
                    output.extend(Self::traverse_tree(&child))
                } else {
                    output.push(child);
                }
            }
            output
        }
        /// Splits a raw string into argument words respecting quotation marks
        /// ```
        /// # use serenity_command_handler::argument::Argument;
        /// let string = r#"this is a string "with quotes in it""#;
        ///
        /// let args = Argument::get_arg_strings(string);
        /// assert_eq!(args, vec!["this","is","a","string","with quotes in it"]);
        /// ```
        pub fn get_arg_strings<'a>(str: &'a str) -> Vec<&'a str> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct SPLITTER {
                __private_field: (),
            }
            #[doc(hidden)]
            static SPLITTER: SPLITTER = SPLITTER {
                __private_field: (),
            };
            impl ::lazy_static::__Deref for SPLITTER {
                type Target = Regex;
                fn deref(&self) -> &Regex {
                    #[inline(always)]
                    fn __static_ref_initialize() -> Regex {
                        Regex::new(r#""(.+)"|(?:\w|\.)+"#).unwrap()
                    }
                    #[inline(always)]
                    fn __stability() -> &'static Regex {
                        static LAZY: ::lazy_static::lazy::Lazy<Regex> =
                            ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for SPLITTER {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            };
            let mut output = Vec::new();
            for capture in SPLITTER.captures_iter(str) {
                if let Some(capture) = capture.get(1) {
                    output.push(capture.as_str());
                } else {
                    output.push(capture.get(0).unwrap().as_str())
                }
            }
            output
        }
        /// Takes a string and traverses the arguments tree to get a argument map and function to run
        pub fn parse_message(
            content: &str,
            tree: &CommandArgumentsTree,
        ) -> Option<(HashMap<String, Self>, CommandFunction)> {
            let func = tree.func;
            let str_args = Self::get_arg_strings(&content[content.find(' ').unwrap_or(0)..]);
            let mut str_args_iter = str_args.iter().peekable();
            let mut args = HashMap::new();
            match &tree.children {
                Some(children) => {
                    match Self::parse_str(&mut str_args_iter, children, &mut args, func) {
                        Some(f) => match tree.func {
                            Some(f) => Some((args, f)),
                            None => Some((args, f)),
                        },
                        None => match tree.func {
                            Some(f) => Some((HashMap::new(), f)),
                            None => None,
                        },
                    }
                }
                None => match func {
                    Some(f) => Some((args, f)),
                    None => None,
                },
            }
        }
        fn parse_str(
            str_args: &mut Peekable<Iter<&str>>,
            branch: &Vec<CommandArguments>,
            map: &mut HashMap<String, Self>,
            func: Option<CommandFunction>,
        ) -> Option<CommandFunction> {
            let mut func = func;
            for argument in branch {
                match argument {
                    CommandArguments::String { name, required, .. } => {
                        let parsed = Self::parse_string(str_args.peek().unwrap_or(&&""));
                        if *required {
                            if str_args.peek().is_none() || parsed.is_err() {
                                return None;
                            }
                        }
                        if parsed.is_ok() {
                            map.insert(
                                name.to_string(),
                                Self::parse_string(str_args.next().unwrap()).unwrap(),
                            );
                        }
                    }
                    CommandArguments::Integer { name, required, .. } => {
                        let parsed = Self::parse_int(str_args.peek().unwrap_or(&&""));
                        if *required {
                            if str_args.peek().is_none() || parsed.is_err() {
                                return None;
                            }
                        }
                        if parsed.is_ok() {
                            map.insert(
                                name.to_string(),
                                Self::parse_int(str_args.next().unwrap()).unwrap(),
                            );
                        }
                    }
                    CommandArguments::Boolean { name, required, .. } => {
                        let parsed = Self::parse_bool(str_args.peek().unwrap_or(&&""));
                        if *required {
                            if str_args.peek().is_none() || parsed.is_err() {
                                return None;
                            }
                        }
                        if parsed.is_ok() {
                            map.insert(
                                name.to_string(),
                                Self::parse_bool(str_args.next().unwrap()).unwrap(),
                            );
                        }
                    }
                    CommandArguments::Channel { name, required, .. } => {
                        let parsed = Self::parse_channel_id(str_args.peek().unwrap_or(&&""));
                        if *required {
                            if str_args.peek().is_none() || parsed.is_err() {
                                return None;
                            }
                        }
                        if parsed.is_ok() {
                            map.insert(
                                name.to_string(),
                                Self::parse_channel_id(str_args.next().unwrap()).unwrap(),
                            );
                        }
                    }
                    CommandArguments::User { name, required, .. } => {
                        let parsed = Self::parse_user_id(str_args.peek().unwrap_or(&&""));
                        if *required {
                            if str_args.peek().is_none() || parsed.is_err() {
                                return None;
                            }
                        }
                        if parsed.is_ok() {
                            map.insert(
                                name.to_string(),
                                Self::parse_user_id(str_args.next().unwrap()).unwrap(),
                            );
                        }
                    }
                    CommandArguments::Role { name, required, .. } => {
                        let parsed = Self::parse_role_id(str_args.peek().unwrap_or(&&""));
                        if *required {
                            if str_args.peek().is_none() || parsed.is_err() {
                                return None;
                            }
                        }
                        if parsed.is_ok() {
                            map.insert(
                                name.to_string(),
                                Self::parse_role_id(str_args.next().unwrap()).unwrap(),
                            );
                        }
                    }
                    CommandArguments::SubCommand {
                        name,
                        required,
                        func,
                        options,
                        ..
                    } => {
                        let next = str_args.peek();
                        if *required {
                            match next {
                                Some(str) => {
                                    if str != &name {
                                        return None;
                                    }
                                }
                                None => return None,
                            }
                        }
                        if next.is_some() {
                            if &name == str_args.peek().unwrap() {
                                str_args.next();
                                func = match options {
                                    Some(v) => Self::parse_str(str_args, v, map, func.clone()),
                                    None => func.clone(),
                                };
                            }
                        }
                    }
                    CommandArguments::SubCommandGroup {
                        name,
                        required,
                        func,
                        options,
                        ..
                    } => {
                        let next = str_args.peek();
                        if *required {
                            match next {
                                Some(str) => {
                                    if str != &name {
                                        return None;
                                    }
                                }
                                None => return None,
                            }
                        }
                        if next.is_some() {
                            if &name == str_args.peek().unwrap() {
                                str_args.next();
                                func = match options {
                                    Some(v) => Self::parse_str(str_args, v, map, func.clone()),
                                    None => func.clone(),
                                };
                            }
                        }
                    }
                }
            }
            func
        }
        fn parse_string(string: &str) -> Result<Self, ()> {
            Ok(Self::String(string.to_string()))
        }
        fn parse_id_int(string: &str) -> Result<u64, ()> {
            match string.parse::<u64>() {
                Ok(u) => Ok(u),
                Err(_) => Err(()),
            }
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const str_parse_test: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("argument::str_parse_test"),
            ignore: false,
            allow_fail: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(str_parse_test())),
    };
    fn str_parse_test() {
        use crate::{
            commands::{ArgumentChoice, CommandArguments, CommandArgumentsTree, CommandResult},
            framework::CommandContext,
        };
        use serenity::{
            futures::future::{BoxFuture, FutureExt},
            model::id::UserId,
        };
        fn test<'fut>(_ctx: &'fut CommandContext) -> BoxFuture<'fut, CommandResult> {
            async move {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["test\n"],
                        &match () {
                            () => [],
                        },
                    ));
                };
                Ok(())
            }
            .boxed()
        }
        fn test2<'fut>(_ctx: &'fut CommandContext) -> BoxFuture<'fut, CommandResult> {
            async move {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["test2\n"],
                        &match () {
                            () => [],
                        },
                    ));
                };
                Ok(())
            }
            .boxed()
        }
        fn test3<'fut>(_ctx: &'fut CommandContext) -> BoxFuture<'fut, CommandResult> {
            async move {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["test3\n"],
                        &match () {
                            () => [],
                        },
                    ));
                };
                Ok(())
            }
            .boxed()
        }
        let arguments_tree = CommandArgumentsTree {
            children: Some(<[_]>::into_vec(box [
                CommandArguments::SubCommandGroup {
                    name: "get",
                    description: "",
                    required: false,
                    func: None,
                    options: Some(<[_]>::into_vec(box [
                        CommandArguments::SubCommand {
                            name: "points",
                            description: "get a user's points",
                            required: false,
                            func: Some(test),
                            options: Some(<[_]>::into_vec(box [CommandArguments::User {
                                name: "user",
                                description: "the selected user",
                                required: true,
                            }])),
                        },
                        CommandArguments::SubCommand {
                            name: "leaderboard",
                            description: "get the guild leaderboard",
                            required: false,
                            func: Some(test2),
                            options: Some(<[_]>::into_vec(box [CommandArguments::Integer {
                                name: "page",
                                description: "the page of the leaderboard to get",
                                required: false,
                                choices: Some(<[_]>::into_vec(box [ArgumentChoice {
                                    name: "default",
                                    value: 0,
                                }])),
                            }])),
                        },
                    ])),
                },
                CommandArguments::SubCommand {
                    name: "self",
                    description: "get your personal stats",
                    required: false,
                    options: None,
                    func: Some(test3),
                },
            ])),
            func: None,
        };
        let points = Argument::parse_message("test get points 100", &arguments_tree);
        let leaderboard = Argument::parse_message("test get leaderboard", &arguments_tree);
        let get_self = Argument::parse_message("test get self", &arguments_tree);
        let get = Argument::parse_message("test get", &arguments_tree);
        if !points.is_some() {
            ::core::panicking::panic("assertion failed: points.is_some()")
        };
        let args = points.unwrap();
        {
            match (&(args.1 as usize), &(test as usize)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        };
        {
            match (&args.0.get("user"), &Some(&Argument::User(UserId(100)))) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        };
        if !leaderboard.is_some() {
            ::core::panicking::panic("assertion failed: leaderboard.is_some()")
        };
        let args = leaderboard.unwrap();
        {
            match (&(args.1 as usize), &(test2 as usize)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        };
        {
            match (&args.0.get("page"), &None) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        };
        if !get_self.is_some() {
            ::core::panicking::panic("assertion failed: get_self.is_some()")
        };
        let args = get_self.unwrap();
        {
            match (&(args.1 as usize), &(test3 as usize)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        };
        if !args.0.is_empty() {
            ::core::panicking::panic("assertion failed: args.0.is_empty()")
        };
        if !get.is_none() {
            ::core::panicking::panic("assertion failed: get.is_none()")
        };
    }
}
/// Everything related to commands.
pub mod commands {
    use std::error::Error;
    use serde_json::json;
    use serenity::futures::future::BoxFuture;
    use serde::Serialize;
    use crate::framework::CommandContext;
    /// A command
    pub struct Command {
        /// The name of the command
        pub name: &'static str,
        /// Description of the command
        pub description: &'static str,
        /// The tree of arguments
        #[serde(flatten)]
        pub arguments_tree: CommandArgumentsTree,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Command {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_map(
                    __serializer,
                    _serde::__private::None,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeMap::serialize_entry(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeMap::serialize_entry(
                    &mut __serde_state,
                    "description",
                    &self.description,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::Serialize::serialize(
                    &&self.arguments_tree,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeMap::end(__serde_state)
            }
        }
    };
    /// A function run for a command
    pub type CommandFunction = fn(&CommandContext) -> BoxFuture<CommandResult>;
    /// The return type of CommandFunction
    pub type CommandResult<T = ()> = Result<T, CommandError>;
    /// Variable error type for commands
    pub type CommandError = Box<dyn Error + Send + Sync>;
    /// The root of the command arguments tree
    ///
    /// Seperated to make distinguishing the root from a node easier as some logic only applies to the root.<br>
    /// Despite this in most ways the root can act like a node.
    pub struct CommandArgumentsTree {
        /// The nodes in the tree
        #[serde(rename = "options")]
        pub children: Option<Vec<CommandArguments>>,
        /// The top level function to run
        ///
        /// Never valid to run if children contains a SubCommand
        #[serde(skip)]
        pub func: Option<CommandFunction>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CommandArgumentsTree {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "CommandArgumentsTree",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "options",
                    &self.children,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    /// The argument metadata we store with the command
    ///
    /// These are used to parse text / interaction responses into [Arguments](crate::argument::Argument)
    #[allow(missing_docs)]
    pub enum CommandArguments {
        SubCommand {
            name: &'static str,
            description: &'static str,
            required: bool,
            options: Option<Vec<CommandArguments>>,
            func: Option<CommandFunction>,
        },
        SubCommandGroup {
            name: &'static str,
            description: &'static str,
            required: bool,
            options: Option<Vec<CommandArguments>>,
            func: Option<CommandFunction>,
        },
        String {
            name: &'static str,
            description: &'static str,
            required: bool,
            choices: Option<Vec<ArgumentChoice<String>>>,
        },
        Integer {
            name: &'static str,
            description: &'static str,
            required: bool,
            choices: Option<Vec<ArgumentChoice<i32>>>,
        },
        Boolean {
            name: &'static str,
            description: &'static str,
            required: bool,
        },
        User {
            name: &'static str,
            description: &'static str,
            required: bool,
        },
        Channel {
            name: &'static str,
            description: &'static str,
            required: bool,
        },
        Role {
            name: &'static str,
            description: &'static str,
            required: bool,
        },
    }
    impl Serialize for CommandArguments {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut map = serde_json::Map::new();
            match self {
                CommandArguments::SubCommand {
                    description,
                    name,
                    required,
                    options,
                    ..
                } => {
                    map.insert("type".to_owned(), ::serde_json::to_value(&1).unwrap());
                    map.insert(
                        "description".to_owned(),
                        ::serde_json::to_value(&description).unwrap(),
                    );
                    map.insert("name".to_owned(), ::serde_json::to_value(&name).unwrap());
                    map.insert(
                        "required".to_owned(),
                        ::serde_json::to_value(&required).unwrap(),
                    );
                    if let Some(t) = options {
                        map.insert("options".to_owned(), ::serde_json::to_value(&t).unwrap());
                    }
                }
                CommandArguments::SubCommandGroup {
                    name,
                    description,
                    required,
                    options,
                    ..
                } => {
                    map.insert("type".to_owned(), ::serde_json::to_value(&2).unwrap());
                    map.insert("name".to_owned(), ::serde_json::to_value(&name).unwrap());
                    map.insert(
                        "description".to_owned(),
                        ::serde_json::to_value(&description).unwrap(),
                    );
                    map.insert(
                        "required".to_owned(),
                        ::serde_json::to_value(&required).unwrap(),
                    );
                    if let Some(t) = options {
                        map.insert("options".to_owned(), ::serde_json::to_value(&t).unwrap());
                    }
                }
                CommandArguments::String {
                    name,
                    description,
                    required,
                    choices,
                    ..
                } => {
                    map.insert("type".to_owned(), ::serde_json::to_value(&3).unwrap());
                    map.insert("name".to_owned(), ::serde_json::to_value(&name).unwrap());
                    map.insert(
                        "description".to_owned(),
                        ::serde_json::to_value(&description).unwrap(),
                    );
                    map.insert(
                        "required".to_owned(),
                        ::serde_json::to_value(&required).unwrap(),
                    );
                    if let Some(t) = choices {
                        map.insert("choices".to_owned(), ::serde_json::to_value(&t).unwrap());
                    }
                }
                CommandArguments::Integer {
                    name,
                    description,
                    required,
                    choices,
                    ..
                } => {
                    map.insert("type".to_owned(), ::serde_json::to_value(&4).unwrap());
                    map.insert("name".to_owned(), ::serde_json::to_value(&name).unwrap());
                    map.insert(
                        "description".to_owned(),
                        ::serde_json::to_value(&description).unwrap(),
                    );
                    map.insert(
                        "required".to_owned(),
                        ::serde_json::to_value(&required).unwrap(),
                    );
                    if let Some(t) = choices {
                        map.insert("choices".to_owned(), ::serde_json::to_value(&t).unwrap());
                    }
                }
                CommandArguments::Boolean {
                    name,
                    description,
                    required,
                    ..
                } => {
                    map.insert("type".to_owned(), ::serde_json::to_value(&5).unwrap());
                    map.insert("name".to_owned(), ::serde_json::to_value(&name).unwrap());
                    map.insert(
                        "description".to_owned(),
                        ::serde_json::to_value(&description).unwrap(),
                    );
                    map.insert(
                        "required".to_owned(),
                        ::serde_json::to_value(&required).unwrap(),
                    );
                }
                CommandArguments::User {
                    name,
                    description,
                    required,
                    ..
                } => {
                    map.insert("type".to_owned(), ::serde_json::to_value(&6).unwrap());
                    map.insert("name".to_owned(), ::serde_json::to_value(&name).unwrap());
                    map.insert(
                        "description".to_owned(),
                        ::serde_json::to_value(&description).unwrap(),
                    );
                    map.insert(
                        "required".to_owned(),
                        ::serde_json::to_value(&required).unwrap(),
                    );
                }
                CommandArguments::Channel {
                    name,
                    description,
                    required,
                    ..
                } => {
                    map.insert("type".to_owned(), ::serde_json::to_value(&7).unwrap());
                    map.insert("name".to_owned(), ::serde_json::to_value(&name).unwrap());
                    map.insert(
                        "description".to_owned(),
                        ::serde_json::to_value(&description).unwrap(),
                    );
                    map.insert(
                        "required".to_owned(),
                        ::serde_json::to_value(&required).unwrap(),
                    );
                }
                CommandArguments::Role {
                    name,
                    description,
                    required,
                    ..
                } => {
                    map.insert("type".to_owned(), ::serde_json::to_value(&8).unwrap());
                    map.insert("name".to_owned(), ::serde_json::to_value(&name).unwrap());
                    map.insert(
                        "description".to_owned(),
                        ::serde_json::to_value(&description).unwrap(),
                    );
                    map.insert(
                        "required".to_owned(),
                        ::serde_json::to_value(&required).unwrap(),
                    );
                }
                #[allow(unreachable_patterns)]
                _ => {}
            };
            map.serialize(serializer)
        }
    }
    /// Represents a choice for an argument
    pub struct ArgumentChoice<T> {
        /// The name of the choice
        pub name: &'static str,
        /// The value of the choice
        pub value: T,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<T> _serde::Serialize for ArgumentChoice<T>
        where
            T: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ArgumentChoice",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "value",
                    &self.value,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
}
/// Everything related to the framework metadata and handlers.
pub mod framework {
    #![allow(dead_code)]
    use std::{collections::HashMap, fmt::Debug};
    use serde_json::Value;
    use serenity::{
        async_trait,
        builder::{CreateEmbed, CreateMessage},
        client::{bridge::gateway::event::ShardStageUpdateEvent, Context, EventHandler},
        futures::future::{BoxFuture, FutureExt},
        http::Http,
        model::{
            channel::{Channel, ChannelCategory, GuildChannel, Message, Reaction},
            event::{
                ChannelPinsUpdateEvent, GuildMembersChunkEvent, InviteCreateEvent,
                InviteDeleteEvent, MessageUpdateEvent, PresenceUpdateEvent, ResumedEvent,
                TypingStartEvent, VoiceServerUpdateEvent,
            },
            guild::{Emoji, Guild, GuildUnavailable, Member, PartialGuild, Role},
            id::{ChannelId, CommandId, EmojiId, GuildId, MessageId, RoleId},
            interactions::{Interaction, InteractionResponseType, InteractionType},
            prelude::{CurrentUser, Presence, Ready, User, VoiceState},
        },
        Result,
    };
    use crate::{argument::Argument, commands::Command, settings::SettingsProvider};
    /// The command framework, holds all commands and settings
    pub struct Framework<T: SettingsProvider> {
        commands: HashMap<&'static str, Command>,
        settings: T,
        /// Stores any additional [EventHandlers](EventHandler) registered
        handlers: Vec<Box<dyn EventHandler>>,
        application_id: u64,
        registered_command_cache: HashMap<String, CommandId>,
    }
    impl<T: SettingsProvider> Framework<T> {
        /// Creates a new Framework
        pub async fn new(settings: T, application_id: u64, token: String) -> Self {
            let http = Http::new_with_token(&token);
            let registered_command_cache = http
                .get_global_application_commands(application_id)
                .await
                .unwrap()
                .iter()
                .map(|a| (a.name.clone(), a.id))
                .collect();
            Framework {
                commands: HashMap::new(),
                settings,
                handlers: Vec::new(),
                application_id,
                registered_command_cache,
            }
        }
        /// Adds a command
        pub fn command<C: CommandInit>(mut self) -> Self {
            let cmd = C::command_init();
            self.commands.insert(cmd.name, cmd);
            self
        }
        /// Adds an [EventHandler] to run alongside the framework.
        ///
        /// Needed because Serenity does not allow more than one EventHandler registered at once and the framework uses it for commands.
        pub fn event_handler<E: EventHandler + 'static>(mut self, handler: E) -> Self {
            self.handlers.push(Box::new(handler));
            self
        }
        /// Register a Command as a slash command
        ///
        /// If `guild_id` is `None` then the command is registered globally
        pub async fn register_slash_command(
            &self,
            http: &Http,
            cmd: &Command,
            guild_id: Option<GuildId>,
        ) -> Result<()> {
            match guild_id {
                Some(g) => {
                    let commmand_cache = http
                        .get_guild_application_commands(self.application_id, g.0)
                        .await
                        .unwrap()
                        .iter()
                        .map(|a| (a.name.clone(), a.id))
                        .collect::<HashMap<String, CommandId>>();
                    if commmand_cache.contains_key(&cmd.name.to_owned()) {
                        http.edit_guild_application_command(
                            self.application_id,
                            g.0,
                            commmand_cache.get(&cmd.name.to_owned()).unwrap().0,
                            &serde_json::to_value(cmd)?,
                        )
                        .await?;
                    } else {
                        http.create_guild_application_command(
                            self.application_id,
                            g.0,
                            &serde_json::to_value(cmd)?,
                        )
                        .await?;
                    }
                }
                None => {
                    if self
                        .registered_command_cache
                        .contains_key(&cmd.name.to_owned())
                    {
                        http.edit_global_application_command(
                            self.application_id,
                            self.registered_command_cache
                                .get(&cmd.name.to_owned())
                                .unwrap()
                                .0,
                            &serde_json::to_value(cmd)?,
                        )
                        .await?;
                    } else {
                        http.create_global_application_command(
                            self.application_id,
                            &serde_json::to_value(cmd)?,
                        )
                        .await?;
                    }
                }
            }
            Ok(())
        }
    }
    /// A trait impl-ed automatically by the command macro to init commands with the framework
    pub trait CommandInit {
        /// The function run to initialize the command
        fn command_init() -> Command;
    }
    /// Stores the source the command was called from
    #[allow(missing_docs)]
    pub enum CommandSource {
        Interaction(Interaction),
        Message(Message),
        #[cfg(test)]
        Test(&'static str),
    }
    /// The context sent to a command's function
    /// Holds arguments, source and Serenity context
    pub struct CommandContext {
        source: CommandSource,
        args: HashMap<String, Argument>,
    }
    impl Debug for CommandContext {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (key, value) in &self.args {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ": ", "\n"],
                    &match (&&key, &&value) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))?;
            }
            Ok(())
        }
    }
    #[cfg(test)]
    impl CommandContext {
        pub fn new(source: CommandSource, args: HashMap<String, Argument>) -> Self {
            CommandContext { args, source }
        }
    }
}
/// Default permission checks and PermissionCheck type
pub mod permissions {
    use serenity::{
        client::Context,
        futures::future::BoxFuture,
        http::Http,
        model::{channel::GuildChannel, guild::Member},
    };
    use crate::commands::CommandResult;
    use serenity_command_handler_macros::permissions_check;
    /// A permissions check
    pub type PermissionsCheck =
        for<'a> fn(&'a Http, &'a Member, &'a GuildChannel) -> BoxFuture<'a, PermsResult>;
    /// The return type of a permissions check
    pub type PermsResult = CommandResult<bool>;
    /// Permission check that passes if the member has the administrator permission
    #[allow(non_snake_case)]
    pub fn ADMINISTRATOR<'fut>(
        ctx: &'fut Context,
        member: &'fut Member,
        _channel: &'fut GuildChannel,
    ) -> ::serenity::futures::future::BoxFuture<'fut, CommandResult<bool>> {
        use ::serenity::futures::future::FutureExt;
        async move {
            {
                Ok(member.permissions(ctx).await?.administrator())
            }
        }
        .boxed()
    }
    /// Permission check that passes if the member has the manage messages permission either globaly or in the channel
    #[allow(non_snake_case)]
    pub fn MANNAGE_MESSAGES<'fut>(
        ctx: &'fut Context,
        member: &'fut Member,
        channel: &'fut GuildChannel,
    ) -> ::serenity::futures::future::BoxFuture<'fut, CommandResult<bool>> {
        use ::serenity::futures::future::FutureExt;
        async move {
            {
                Ok(member.permissions(ctx).await?.manage_messages()
                    || channel
                        .permissions_for_user(ctx, member.user.id)
                        .await?
                        .manage_messages())
            }
        }
        .boxed()
    }
}
/// The settings for the framework.
pub mod settings {
    use std::sync::Arc;
    use serenity::{
        futures::{lock::Mutex, FutureExt},
        model::id::GuildId,
    };
    /// Allows users to define custom settings providers for the handler to pull from.
    pub trait SettingsProvider {
        /// The default prefixes the bot should fallback to.
        fn default_prefixes(&self) -> Vec<String>;
        /// The prefixes for a specific guild.
        fn prefixes(&self, guild_id: GuildId) -> Option<Vec<String>>;
        /// Whether we should auto-register.
        fn auto_register(&self) -> bool;
        /// Whether we should auto-delete non-existant commands.
        fn auto_delete(&self) -> bool;
        /// Guilds to register commands to.
        fn auto_register_guilds(&self) -> Vec<GuildId>;
    }
    /// Represents the settings for the framework
    pub struct Settings {
        /// The prefixes the bot uses
        pub prefixes: Vec<&'static str>,
        /// Whether to auto-register commands as slash commands on Ready.
        pub auto_register: bool,
        /// Whether to auto-delete unrecognized slash commands on Ready.
        pub auto_delete: bool,
        /// Guilds to register slash commands to.
        ///
        /// Registers all commands to these guilds regardles of `auto_register`.<br>
        /// Is useful for quick updates or if you have `auto_register` off.
        pub slash_command_guilds: Vec<GuildId>,
    }
    impl SettingsProvider for Settings {
        fn default_prefixes(&self) -> Vec<String> {
            self.prefixes.iter().map(|s| s.to_string()).collect()
        }
        fn prefixes(&self, _guild_id: GuildId) -> Option<Vec<String>> {
            Some(self.default_prefixes())
        }
        fn auto_register(&self) -> bool {
            self.auto_register
        }
        fn auto_delete(&self) -> bool {
            self.auto_delete
        }
        fn auto_register_guilds(&self) -> Vec<GuildId> {
            self.slash_command_guilds.clone()
        }
    }
    impl<T: SettingsProvider + Send> SettingsProvider for Arc<Mutex<T>> {
        fn default_prefixes(&self) -> Vec<String> {
            async { self.lock().await.default_prefixes() }
                .now_or_never()
                .unwrap()
        }
        fn prefixes(&self, guild_id: GuildId) -> Option<Vec<String>> {
            async { self.lock().await.prefixes(guild_id) }
                .now_or_never()
                .unwrap()
        }
        fn auto_register(&self) -> bool {
            async { self.lock().await.auto_register() }
                .now_or_never()
                .unwrap()
        }
        fn auto_delete(&self) -> bool {
            async { self.lock().await.auto_delete() }
                .now_or_never()
                .unwrap()
        }
        fn auto_register_guilds(&self) -> Vec<GuildId> {
            async { self.lock().await.auto_register_guilds() }
                .now_or_never()
                .unwrap()
        }
    }
    impl<T: SettingsProvider> SettingsProvider for Arc<T> {
        fn default_prefixes(&self) -> Vec<String> {
            T::default_prefixes(&self)
        }
        fn prefixes(&self, guild_id: GuildId) -> Option<Vec<String>> {
            T::prefixes(&self, guild_id)
        }
        fn auto_register(&self) -> bool {
            T::auto_register(&self)
        }
        fn auto_delete(&self) -> bool {
            T::auto_delete(&self)
        }
        fn auto_register_guilds(&self) -> Vec<GuildId> {
            T::auto_register_guilds(&self)
        }
    }
}
#[cfg(test)]
mod tests {
    #[cfg(test)]
    pub mod test {
        use std::collections::HashMap;
        use serenity::FutureExt;
        use crate::{
            argument::Argument,
            command,
            commands::CommandResult,
            framework::{CommandContext, CommandSource},
            subcommand,
        };
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const subcommand_test: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("tests::test::subcommand_test"),
                ignore: false,
                allow_fail: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(subcommand_test())),
        };
        fn subcommand_test() {
            fn test<'fut>(
                _ctx: &'fut CommandContext,
            ) -> ::serenity::futures::future::BoxFuture<'fut, CommandResult<u32>> {
                use ::serenity::futures::future::FutureExt;
                async move {
                    const _: fn() = || {
                        trait TypeEq {
                            type This: ?Sized;
                        }
                        impl<T: ?Sized> TypeEq for T {
                            type This = Self;
                        }
                        fn assert_type_eq_all<T, U>()
                        where
                            T: ?Sized + TypeEq<This = U>,
                            U: ?Sized,
                        {
                        }
                    };
                    const _: fn() = || {
                        trait TypeEq {
                            type This: ?Sized;
                        }
                        impl<T: ?Sized> TypeEq for T {
                            type This = Self;
                        }
                        fn assert_type_eq_all<T, U>()
                        where
                            T: ?Sized + TypeEq<This = U>,
                            U: ?Sized,
                        {
                        }
                    };
                    {
                        Ok(5);
                    }
                }
                .boxed()
            }
            let x = test(&CommandContext::new(
                CommandSource::Test(""),
                HashMap::new(),
            ))
            .now_or_never()
            .unwrap()
            .unwrap();
            {
                match (&x, &5) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&&*left_val, &&*right_val) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                    ],
                                },
                            ))
                        }
                    }
                }
            };
        }
    }
}
/// Macro to create commands.
///
/// # Command macro format
/// ```text
/// command!{
///     name,
///     description,
///     function,
///     [
///         required Type name = function | "description" {choices: map} [children]
///     ]
/// }
/// ```
///
/// ## Argument Types
/// - Integer (u32)
/// - Str (String)
/// - Boolean (bool)
/// - User (UserId)
/// - Channel (ChannelId)
/// - Role (RoleId)
/// - SubCommand
/// - SubCommandGroup
///
/// ### Type Specific Fields
/// The fields `children`, `choices`, and `function` are all only valid for some of the argument types.<br>
/// Only SubCommand and SubCommandGroup arguments can have `function` or `children`.<br>
/// And only Integer and Str can have `choices`.
///
/// ## SubCommands
/// SubCommands allow you to have multiple paths to your command.
///
/// By including a function when defining a SubCommand that function will be run when the SubCommand is given.
///
/// SubCommands can also have child arguments and their functions will only be run when all required arguments are present.
///
/// ## Examples
/// `stats get|set [user]`
/// ```
/// # use serenity_command_handler_macros::*;
/// # use serenity_command_handler::commands::*;
/// # use serenity_command_handler::framework::*;
/// # #[subcommand]
/// # pub async fn points(_ctx: &CommandContext) -> CommandResult {Ok(())}
/// # #[subcommand]
/// # pub async fn credits(_ctx: &CommandContext) -> CommandResult {Ok(())}
/// # #[subcommand]
/// # pub async fn set_stats(_ctx: &CommandContext) -> CommandResult {Ok(())}
/// command!{
///     stats,
///     "get or set a user's stats",
///     [
///         optional SubCommandGroup get | "get info about stats" [
///             optional SubCommand points = points | "get a user's points" [
///                 optional User user | "the user whose points you want to get"
///             ],
///             optional SubCommand credits = credits | "get a user's credits" [
///                 optional User user | "the user whose points you want to get"
///             ]
///         ],
///         optional SubCommand set = set_stats | "set a user's stats"
///     ]
/// }
/// ```
///
/// `add a b`
/// ```
/// # use serenity_command_handler_macros::*;
/// # use serenity_command_handler::commands::*;
/// # use serenity_command_handler::framework::*;
/// # #[subcommand]
/// # pub async fn add(_ctx: &CommandContext) -> CommandResult {Ok(())}
/// command!{
///     add,
///     "adds two numbers",
///     add,
///     [
///         required Integer a | "the first number to add",
///         required Integer b | "the second number to add"
///     ]
/// }
/// ```
///
/// `grid size` with choices of 1, 5, and 12
/// ```
/// # use serenity_command_handler_macros::*;
/// # use serenity_command_handler::commands::*;
/// # use serenity_command_handler::framework::*;
/// # #[subcommand]
/// # pub async fn grid(_ctx: &CommandContext) -> CommandResult {Ok(())}
/// command!{
///     grid,
///     "prints a grid",
///     grid,
///     [
///         required Integer size | "the size of the grid" {"small": 1, "medium": 5, "large": 12},
///     ]
/// }
/// ```
///
/// You have to follow all the rules of normal discord slash commands.<br>
/// This includes not allowing required arguments after optional ones.
///
/// ```compile_fail
/// # use serenity_command_handler_macros::*;
/// # use serenity_command_handler::commands::*;
/// # use serenity_command_handler::framework::*;
/// # #[subcommand]
/// # pub async fn test(_ctx: &CommandContext) -> CommandResult {Ok(())}
/// command!{
///     test,
///     "testing",
///     test
///    [
///        required Integer test | "the first argument",
///        optional String test2 | "the second argument",
///        required String test3 | "the third argument" // Fails to compile
///     ]
/// }
///```
pub use serenity_command_handler_macros::command;
/// Denotes a function that is used in a command.
///
/// Functions must take a [CommandContext](crate::framework::CommandContext) and return a [CommandResult](crate::commands::CommandResult).
pub use serenity_command_handler_macros::subcommand;
/// Denotes a function that is used to check permissions before running a command.
pub use serenity_command_handler_macros::permissions_check;
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&str_parse_test, &subcommand_test])
}
