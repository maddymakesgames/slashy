#![warn(missing_docs)]

//! A command framework for Serenity that allows commands to be registered both as a traditional text command and a discord slash command

/// Everything related to parsing and representing arguments.
pub mod argument;
/// Everything related to commands.
pub mod commands;
/// Everything related to the framework metadata and handlers.
pub mod framework;
/// Default permission checks and PermissionCheck type
pub mod permissions;
/// The settings for the framework.
pub mod settings;
#[cfg(test)]
mod tests;

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
pub use slashy_macros::command;

/// Denotes a function that is used in a command.
///
/// Functions must take a [CommandContext](crate::framework::CommandContext) and return a [CommandResult](crate::commands::CommandResult).
pub use slashy_macros::subcommand;

/// Denotes a function that is used to check permissions before running a command.
pub use slashy_macros::permissions_check;
