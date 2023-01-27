use serenity::{
    client::Context,
    futures::future::BoxFuture,
    http::Http,
    model::{channel::GuildChannel, guild::Member},
};

use crate::commands::CommandResult;

use slashy_macros::permissions_check;

/// A permissions check
pub type PermissionsCheck =
    for<'a> fn(&'a Http, &'a Member, &'a GuildChannel) -> BoxFuture<'a, PermsResult>;
/// The return type of a permissions check
pub type PermsResult = CommandResult<bool>;


/// Permission check that passes if the member has the administrator permission
#[allow(non_snake_case)]
#[permissions_check]
pub async fn ADMINISTRATOR(
    ctx: &Context,
    member: &Member,
    _channel: &GuildChannel,
) -> CommandResult<bool> {
    Ok(member.permissions(ctx)?.administrator())
}

/// Permission check that passes if the member has the manage messages permission either globaly or in the channel
#[allow(non_snake_case)]
#[permissions_check]
pub async fn MANNAGE_MESSAGES(
    ctx: &Context,
    member: &Member,
    channel: &GuildChannel,
) -> CommandResult<bool> {
    Ok(member.permissions(ctx)?.manage_messages()
        || channel
            .permissions_for_user(ctx, member.user.id)?
            .manage_messages())
}
