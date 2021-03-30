use std::error::Error;

use serde_json::json;
use serenity::futures::future::BoxFuture;


use serde::Serialize;

use crate::framework::CommandContext;

/// A command
#[derive(Serialize)]
pub struct Command {
    /// The name of the command
    pub name: &'static str,
    /// Description of the command
    pub description: &'static str,
    /// The tree of arguments
    #[serde(flatten)]
    pub arguments_tree: CommandArgumentsTree,
}

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
#[derive(Serialize)]
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


macro_rules! command_options_serialize {
    ($self: ident, $map: ident, $($val: path, $type_val: expr, $( $i:ident),* | $($i1:ident),*);*) => {
        match $self {
            $(
            $val { $($i,)* $($i1,)* ..} => {
                $map.insert("type".to_owned(), json!($type_val));
                $($map.insert(stringify!($i).to_owned(), json!($i)));*;
                $(if let Some(t) = $i1 {
                    $map.insert(stringify!($i1).to_owned(), json!(t));
                })*
            }),*
            #[allow(unreachable_patterns)]
            _ => {}
        }
    };
}

impl Serialize for CommandArguments {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serde_json::Map::new();

        command_options_serialize!(
            self, map,
            CommandArguments::SubCommand, 1, description, name, required | options;
            CommandArguments::SubCommandGroup, 2, name, description, required | options;
            CommandArguments::String, 3, name, description, required| choices;
            CommandArguments::Integer, 4, name, description, required| choices;
            CommandArguments::Boolean, 5, name, description, required|;
            CommandArguments::User, 6, name, description, required|;
            CommandArguments::Channel, 7, name, description, required|;
            CommandArguments::Role, 8, name, description, required|
        );

        map.serialize(serializer)
    }
}

#[derive(Serialize)]
/// Represents a choice for an argument
pub struct ArgumentChoice<T> {
    /// The name of the choice
    pub name: &'static str,
    /// The value of the choice
    pub value: T,
}
