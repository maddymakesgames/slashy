// Allow dead code as the impl of CommandContext is a public facing api and so would mostly be dead in the lib itself
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
            ChannelPinsUpdateEvent,
            GuildMembersChunkEvent,
            InviteCreateEvent,
            InviteDeleteEvent,
            MessageUpdateEvent,
            PresenceUpdateEvent,
            ResumedEvent,
            TypingStartEvent,
            VoiceServerUpdateEvent,
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

/// Generates event functions that run any other EventHandlers registered
macro_rules! event_handler_runners {
    ($($func: ident, $($var_name: ident, $type: ty),*);*) => {
        $(fn $func<'life0, 'async_trait, >(&'life0 self, ctx: Context, $($var_name: $type),*)
        -> BoxFuture<'async_trait,()>
        where
        'life0: 'async_trait,
        Self: 'async_trait,
        {
            async move {
                for handler in &self.handlers {
                    handler.$func(ctx.clone(), $($var_name.clone()),*).await;
                }
            }.boxed()
        })*
    };
    ($($func: ident, $($l: lifetime),*; $($var_name: ident, $type: ty),*);*) => {
        $(fn $func<'life0, $($l,)* 'async_trait>(&'life0 self, ctx: Context, $($var_name: $type),* )
        -> BoxFuture<'async_trait, ()>
        where 'life0: 'async_trait, $($l: 'async_trait,)* Self: 'async_trait {
            async move {
                for handler in &self.handlers {
                    handler.$func(ctx.clone(), $($var_name),*).await;
                }
            }.boxed()
        })*
    }
}


#[async_trait]
impl<T: SettingsProvider + Send + Sync> EventHandler for Framework<T> {
    // Run any other EventHandlers we have registered
    event_handler_runners! {
        cache_ready, e, Vec<GuildId>;
        channel_pins_update, e, ChannelPinsUpdateEvent;
        channel_update, o, Option<Channel>, n, Channel;
        guild_ban_addition, g, GuildId, b, User;
        guild_ban_removal, g, GuildId, b, User;
        guild_create, g, Guild, i, bool;
        guild_delete, i, GuildUnavailable, f, Option<Guild>;
        guild_emojis_update, g, GuildId, c, HashMap<EmojiId, Emoji>;
        guild_integrations_update, g, GuildId;
        guild_member_addition, g, GuildId, m, Member;
        guild_member_removal, g, GuildId, u, User, m, Option<Member>;
        guild_member_update, o, Option<Member>, n, Member;
        guild_members_chunk, c, GuildMembersChunkEvent;
        guild_role_create, g, GuildId, n, Role;
        guild_role_delete, g, GuildId, r, RoleId, ro, Option<Role>;
        guild_role_update, g, GuildId, o, Option<Role>, n, Role;
        guild_unavailable, g, GuildId;
        guild_update, o, Option<Guild>, n, PartialGuild;
        invite_create, d, InviteCreateEvent;
        invite_delete, d, InviteDeleteEvent;
        message_delete, c, ChannelId, d, MessageId, g, Option<GuildId>;
        message_delete_bulk, c, ChannelId, m, Vec<MessageId>, g, Option<GuildId>;
        message_update, o, Option<Message>, n, Option<Message>, e, MessageUpdateEvent;
        reaction_add, a, Reaction;
        reaction_remove, r, Reaction;
        reaction_remove_all, c, ChannelId, r, MessageId;
        presence_replace, a, Vec<Presence>;
        presence_update, n, PresenceUpdateEvent;
        resume, a, ResumedEvent;
        shard_stage_update, a, ShardStageUpdateEvent;
        typing_start, a, TypingStartEvent;
        unknown, n, String, a, Value;
        user_update, o, CurrentUser, n, CurrentUser;
        voice_server_update, a, VoiceServerUpdateEvent;
        voice_state_update, a, Option<GuildId>, o, Option<VoiceState>, n, VoiceState;
        webhook_update, g, GuildId, b, ChannelId
    }

    event_handler_runners! {
        channel_create, 'a; e, &'a GuildChannel;
        category_create, 'a; e, &'a ChannelCategory;
        category_delete, 'a; e, &'a ChannelCategory;
        channel_delete, 'a; e, &'a GuildChannel
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        for handler in &self.handlers {
            handler.ready(ctx.clone(), data_about_bot.clone()).await
        }

        // Allow unwraps here because we *should* panic if these fail

        if self.settings.auto_delete() {
            for (reg_name, reg_id) in &self.registered_command_cache {
                if !self.commands.contains_key(reg_name.as_str()) {
                    ctx.http
                        .delete_global_application_command(self.application_id, reg_id.0)
                        .await
                        .unwrap();
                }
            }
        }

        if self.settings.auto_register() {
            for (_, cmd) in &self.commands {
                self.register_slash_command(&ctx.http, cmd, None)
                    .await
                    .unwrap()
            }
        }

        for guild_id in self.settings.auto_register_guilds() {
            for (_, cmd) in &self.commands {
                self.register_slash_command(&ctx.http, cmd, Some(guild_id))
                    .await
                    .unwrap()
            }
        }
    }

    async fn message(&self, ctx: Context, message: Message) {
        // Run any other handlers registered
        for handler in &self.handlers {
            handler.message(ctx.clone(), message.clone()).await
        }

        if message.author.bot {
            return;
        }

        let mut found_prefix = String::new();

        let prefix_list = match self
            .settings
            .prefixes(message.guild_id.unwrap_or(GuildId(0)))
        {
            Some(v) => v,
            None => self.settings.default_prefixes(),
        };

        for prefix in prefix_list {
            if message.content.starts_with(&prefix) {
                found_prefix = prefix.to_string();
                break;
            }
        }

        if found_prefix == String::new() {
            return;
        }

        let cropped_msg = &message.content[found_prefix.len() ..].to_owned();

        let cmd_str = cropped_msg.split(" ").next().unwrap_or_default();

        match self.commands.get(cmd_str) {
            Some(cmd) => {
                #[cfg(debug_assertions)]
                let source = CommandSource::Message(message.clone());
                #[cfg(not(debug_assertions))]
                // Don't clone message if we aren't using it later
                let source = CommandSource::Message(message);

                match Argument::parse(&source, &cmd.arguments_tree) {
                    Some((args, func)) => {
                        #[cfg(debug_assertions)]
                        let context = CommandContext::new(ctx.clone(), source, args);
                        #[cfg(not(debug_assertions))]
                        // Don't clone ctx if we don't need to
                        let context = CommandContext::new(ctx, source, args);
                        match func(&context).await {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("{:?}", e);
                                #[cfg(debug_assertions)]
                                // message sends should only fail on perm errors or too many chars
                                // neither *should* occur while testing
                                message
                                    .channel_id
                                    .send_message(ctx, |m| m.content(format!("Error: {}", e)))
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                    // Don't do anything if arguments are wrong
                    // Maybe handle later
                    None => {}
                }
            }
            // If we don't have a command do nothing
            // In the future we could have a handler for this but generally its best practice to not
            None => {}
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Run any other handlers registered
        for handler in &self.handlers {
            handler
                .interaction_create(ctx.clone(), interaction.clone())
                .await
        }

        if interaction.kind != InteractionType::ApplicationCommand {
            return;
        }

        let name = match &interaction.data {
            Some(data) => data.name.to_owned(),
            // Should never be reached if we have a command interaction
            // All commands *should* come with data
            None => unreachable!(),
        };

        match self.commands.get(name.as_str()) {
            Some(cmd) => {
                #[cfg(debug_assertions)]
                let source = CommandSource::Interaction(interaction.clone());
                #[cfg(not(debug_assertions))]
                let source = CommandSource::Interaction(interaction);
                match Argument::parse(&source, &cmd.arguments_tree) {
                    Some((args, func)) => {
                        #[cfg(debug_assertions)]
                        let context = CommandContext::new(ctx.clone(), source, args);
                        #[cfg(not(debug_assertions))]
                        // Don't clone ctx if we don't need to
                        let context = CommandContext::new(ctx, source, args);
                        match func(&context).await {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("{:?}", e);
                                #[cfg(debug_assertions)]
                                interaction
                                    .channel_id
                                    .send_message(ctx, |m| m.content(e))
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                    // Do nothing rn
                    None => {
                        interaction
                            .channel_id
                            .send_message(ctx, |m| {
                                m.content(format!("Invalid arguments for command {}", cmd.name))
                            })
                            .await
                            .unwrap();
                    }
                }
            },
            None => println!("We got command `{}` which is not registered.\nMost likely the global command cache has not updated.", name),
        }
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
    #[cfg(not(test))]
    /// The Serenity context that was with the event
    pub ctx: Context,
    source: CommandSource,
    args: HashMap<String, Argument>,
}

#[cfg(not(test))]
impl CommandContext {
    /// Creates a new CommandContext
    pub(crate) fn new(
        ctx: Context,
        source: CommandSource,
        args: HashMap<String, Argument>,
    ) -> Self {
        CommandContext { ctx, args, source }
    }

    /// Gets an argument
    pub fn get_arg<'a>(&'a self, key: &str) -> Option<&'a Argument> {
        self.args.get(key)
    }

    /// Gets the User that triggered the command
    pub fn author(&self) -> &User {
        match &self.source {
            CommandSource::Interaction(i) => &i.member.user,
            CommandSource::Message(m) => &m.author,
        }
    }

    pub async fn send_str<F>(&self, content: &str) -> Result<()> {
        match &self.source {
            CommandSource::Interaction(i) =>
                i.create_interaction_response(&self.ctx, |c| {
                    c.kind(InteractionResponseType::ChannelMessageWithSource);
                    c.interaction_response_data(|n| {
                        n.content(content);

                        n
                    });

                    c
                })
                .await,
            CommandSource::Message(m) => {
                m.channel_id
                    .send_message(&self.ctx, |c| {
                        c.content(content);

                        c
                    })
                    .await?;
                Ok(())
            }
        }
    }

    /// Sends a message to the channel the command was triggered in
    ///
    /// For [Message](CommandSource::Message) this is just a wrapper around [send_message](CommandContext#send_message)
    ///
    /// For [Interaction](CommandSource::Interaction) this sends a [ChannelMessageWithSource](InteractionResponseType::ChannelMessageWithSource)
    ///
    /// Note, we have `content` be an Option<&str> instead of using a CreateMessage callback as interaction responses use different create message types
    pub async fn send_embed<F>(&self, embed: F) -> Result<()>
    where F: Fn(&mut CreateEmbed) -> &mut CreateEmbed {
        match &self.source {
            CommandSource::Interaction(i) =>
                i.create_interaction_response(&self.ctx, |c| {
                    c.kind(InteractionResponseType::ChannelMessageWithSource);
                    c.interaction_response_data(|n| {
                        n.embed(embed);

                        n
                    });

                    c
                })
                .await,
            CommandSource::Message(m) => {
                m.channel_id
                    .send_message(&self.ctx, |c| {
                        c.embed(embed);

                        c
                    })
                    .await?;
                Ok(())
            }
        }
    }

    /// Sends a message to the channel the CommandSource is from
    pub async fn send_message<'a, F>(&self, f: F) -> Result<Message>
    where for<'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a> {
        match &self.source {
            CommandSource::Interaction(i) => i.channel_id.send_message(&self.ctx, f).await,
            CommandSource::Message(m) => m.channel_id.send_message(&self.ctx, f).await,
        }
    }

    /// Gets the member who triggered the command
    pub async fn member(&self) -> Result<Member> {
        match &self.source {
            CommandSource::Interaction(i) => Ok(i.member.clone()),
            CommandSource::Message(m) => m.member(&self.ctx).await,
        }
    }

    /// Gets the guild the command was triggered in
    pub async fn guild(&self) -> Result<PartialGuild> {
        match &self.source {
            CommandSource::Interaction(i) => self.ctx.http.get_guild(i.guild_id.0).await,
            CommandSource::Message(m) => match m.guild_id {
                Some(g) => self.ctx.http.get_guild(g.0).await,
                None => Err(serenity::Error::Other("Called guild() without a guild_id")),
            },
        }
    }

    /// Gets the channel the command was triggered in
    pub async fn channel(&self) -> Result<Channel> {
        match &self.source {
            CommandSource::Interaction(i) => i.channel_id.to_channel(&self.ctx).await,
            CommandSource::Message(m) => m.channel_id.to_channel(&self.ctx).await,
        }
    }

    // pub async fn data_mut<'a>(&'a self) -> RwLockWriteGuard<'a, TypeMap> {
    //     self.data.write().await
    // }
}

impl Debug for CommandContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.args {
            f.write_fmt(format_args!("{}: {:?}\n", &key, &value))?;
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
