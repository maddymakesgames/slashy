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

#[derive(Debug, PartialEq)]
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

macro_rules! cmp_arg_interaction {
    ($branch:ident, $recieved: ident, $hash_map: ident, $arg: ident, $curr_arg: ident, $func: ident,
    $($arg_type: ident, $self_arg: ident, $parser: block);* |
    $($subcommand_type: ident),*
    ) => {
        match $arg {
            $(CommandArguments::$arg_type {name, required, ..} => {
                if &$curr_arg.name == name {
                    match $parser {
                        Some(v) => {
                            $hash_map.insert(name.to_string(), Argument::$self_arg(v.to_owned()));
                            if $recieved.peek().is_none() {
                                break;
                            }
                            $curr_arg = $recieved.next().unwrap();
                        },
                        None => if *required {
                            return None
                        }
                    }
                }
            }),*
            $(CommandArguments::$subcommand_type {name, required, func, options, ..} => {
                if &$curr_arg.name == name {
                    $func = func.clone();
                    if let Some(children) = options {
                        $func = Self::parse_interaction_tree(children, $recieved, $hash_map, $func);
                    }
                } else {
                    if *required {
                        return None
                    }
                }
            },)*
            #[allow(unreachable_patterns)]
            _ => {}
        }
    };
}

macro_rules! arg_message {
    ($str_args: ident, $branch: ident, $map: ident, $func: ident, $arg: ident,
    $($arg_type: ident, $self_arg: ident, $parser: ident);* |
    $($sub_command_type: ident),*) => {
        match $arg {
            $(CommandArguments::$arg_type {name, required, ..} => {
                let parsed = Self::$parser($str_args.peek().unwrap_or(&&""));
                if *required {
                    if $str_args.peek().is_none() || parsed.is_err() {
                        return None
                    }
                }

                if parsed.is_ok() {
                    $map.insert(name.to_string(), Self::$parser($str_args.next().unwrap()).unwrap());
                }
            },)*
            $(CommandArguments::$sub_command_type {name, required, func, options, ..} => {
                let next = $str_args.peek();
                if *required {
                    match next {
                        Some(str) => {
                            if str != &name {
                                return None
                            }
                        },
                        None => return None
                    }
                }

                if next.is_some() {
                    if &name == $str_args.peek().unwrap() {
                        $str_args.next();
                        $func = match options {
                            Some(v) => Self::parse_str($str_args, v, $map, func.clone()),
                            None => func.clone()
                        };
                    }
                }
            },)*
        }
    };
}

macro_rules! parse_string {
    ($($name: ident, $arg_type: ident, $parse_type: ty);* |
    $($name_id: ident, $id_type: ident, $self_type: ident);*) => {
        $(fn $name(string: &str) -> Result<Self, ()> {
            match string.parse::<$parse_type>() {
                Ok(i) => Ok(Argument::$arg_type(i)),
                Err(_) => Err(()),
            }
        })*
        $(fn $name_id(string: &str) -> Result<Self, ()> {
            Ok(Argument::$self_type($id_type(Self::parse_id_int(string)?)))
        })*
    };
}

impl Argument {
    parse_string! {
        parse_int, Integer, i32;
        parse_bool, Boolean, bool |
        parse_role_id, RoleId, Role;
        parse_user_id, UserId, User;
        parse_channel_id, ChannelId, Channel
    }

    /// Traverses the argument tree of `cmd` and outputs a map of arguments and the function to run
    pub fn parse(
        source: &CommandSource,
        tree: &CommandArgumentsTree,
    ) -> Option<(HashMap<String, Self>, CommandFunction)> {
        match source {
            CommandSource::Interaction(interaction) =>
                Argument::parse_interaction(interaction, tree),
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
            // unwrap safe cause None would go to the first branch
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
            cmp_arg_interaction!(
                branch, recieved, map, arg, curr_arg, fun,
                String, String, {curr_arg.value.as_ref().unwrap().as_str()};
                Integer, Integer, {curr_arg.value.as_ref().unwrap().as_i64().map(|u| u as i32)};
                Boolean, Boolean, {curr_arg.value.as_ref().unwrap().as_bool()};
                User, User, {curr_arg.value.as_ref().unwrap().as_i64().map(|u| UserId(u as u64))};
                Channel, Channel, {curr_arg.value.as_ref().unwrap().as_i64().map(|u| ChannelId(u as u64))};
                Role, Role, {curr_arg.value.as_ref().unwrap().as_i64().map(|u| RoleId(u as u64))} |
                SubCommand, SubCommandGroup
            );
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
    /// # use slashy::argument::Argument;
    /// let string = r#"this is a string "with quotes in it""#;
    ///
    /// let args = Argument::get_arg_strings(string);
    /// assert_eq!(args, vec!["this","is","a","string","with quotes in it"]);
    /// ```
    pub fn get_arg_strings<'a>(str: &'a str) -> Vec<&'a str> {
        lazy_static::lazy_static! {
            static ref SPLITTER: Regex = Regex::new(r#""(.+)"|(?:\w|\.)+"#).unwrap();
        };

        let mut output = Vec::new();

        for capture in SPLITTER.captures_iter(str) {
            // Capture group 1 is everything in quotes, 0 is all text captured
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
        let str_args = Self::get_arg_strings(&content[content.find(' ').unwrap_or(0) ..]);
        let mut str_args_iter = str_args.iter().peekable();
        let mut args = HashMap::new();

        match &tree.children {
            Some(children) =>
                match Self::parse_str(&mut str_args_iter, children, &mut args, func) {
                    Some(f) => match tree.func {
                        Some(f) => Some((args, f)),
                        None => Some((args, f)),
                    },
                    None => match tree.func {
                        Some(f) => Some((HashMap::new(), f)),
                        None => None,
                    },
                },
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
            arg_message!(
                str_args, branch, map, func, argument,
                String, String, parse_string;
                Integer, Integer, parse_int;
                Boolean, Bool, parse_bool;
                Channel, Channel, parse_channel_id;
                User, User, parse_user_id;
                Role, Role, parse_role_id |
                SubCommand, SubCommandGroup
            )
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

#[test]
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
            println!("test");
            Ok(())
        }
        .boxed()
    }
    fn test2<'fut>(_ctx: &'fut CommandContext) -> BoxFuture<'fut, CommandResult> {
        async move {
            println!("test2");
            Ok(())
        }
        .boxed()
    }
    fn test3<'fut>(_ctx: &'fut CommandContext) -> BoxFuture<'fut, CommandResult> {
        async move {
            println!("test3");
            Ok(())
        }
        .boxed()
    }

    let arguments_tree = CommandArgumentsTree {
        children: Some(vec![
            CommandArguments::SubCommandGroup {
                name: "get",
                description: "",
                required: false,
                func: None,
                options: Some(vec![
                    CommandArguments::SubCommand {
                        name: "points",
                        description: "get a user's points",
                        required: false,
                        func: Some(test),
                        options: Some(vec![CommandArguments::User {
                            name: "user",
                            description: "the selected user",
                            required: true,
                        }]),
                    },
                    CommandArguments::SubCommand {
                        name: "leaderboard",
                        description: "get the guild leaderboard",
                        required: false,
                        func: Some(test2),
                        options: Some(vec![CommandArguments::Integer {
                            name: "page",
                            description: "the page of the leaderboard to get",
                            required: false,
                            choices: Some(vec![ArgumentChoice {
                                name: "default",
                                value: 0,
                            }]),
                        }]),
                    },
                ]),
            },
            CommandArguments::SubCommand {
                name: "self",
                description: "get your personal stats",
                required: false,
                options: None,
                func: Some(test3),
            },
        ]),
        func: None,
    };

    let points = Argument::parse_message("test get points 100", &arguments_tree);
    let leaderboard = Argument::parse_message("test get leaderboard", &arguments_tree);
    let get_self = Argument::parse_message("test get self", &arguments_tree);
    let get = Argument::parse_message("test get", &arguments_tree);

    assert!(points.is_some());
    let args = points.unwrap();
    assert_eq!(args.1 as usize, test as usize);
    assert_eq!(args.0.get("user"), Some(&Argument::User(UserId(100))));

    assert!(leaderboard.is_some());
    let args = leaderboard.unwrap();
    assert_eq!(args.1 as usize, test2 as usize);
    assert_eq!(args.0.get("page"), None);

    assert!(get_self.is_some());
    let args = get_self.unwrap();
    assert_eq!(args.1 as usize, test3 as usize);
    assert!(args.0.is_empty());

    assert!(get.is_none());
}
