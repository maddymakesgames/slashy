use std::sync::{Arc, Mutex as StdMutex, RwLock as StdRwLock};

use serenity::{
    futures::{lock::Mutex, FutureExt},
    model::id::GuildId,
    prelude::RwLock,
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

// Generic impls for commonly used wrapper types
// Add more as needed but I don't think we should need more than Arc Mutex and RwLock currently


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

impl<T: SettingsProvider + Send> SettingsProvider for Arc<StdMutex<T>> {
    fn default_prefixes(&self) -> Vec<String> {
        T::default_prefixes(&self.lock().unwrap())
    }

    fn prefixes(&self, guild_id: GuildId) -> Option<Vec<String>> {
        T::prefixes(&self.lock().unwrap(), guild_id)
    }

    fn auto_register(&self) -> bool {
        T::auto_register(&self.lock().unwrap())
    }

    fn auto_delete(&self) -> bool {
        T::auto_delete(&self.lock().unwrap())
    }

    fn auto_register_guilds(&self) -> Vec<GuildId> {
        T::auto_register_guilds(&self.lock().unwrap())
    }
}

impl<T: SettingsProvider> SettingsProvider for Arc<T> {
    fn default_prefixes(&self) -> Vec<String> {
        T::default_prefixes(self)
    }

    fn prefixes(&self, guild_id: GuildId) -> Option<Vec<String>> {
        T::prefixes(self, guild_id)
    }

    fn auto_register(&self) -> bool {
        T::auto_register(self)
    }

    fn auto_delete(&self) -> bool {
        T::auto_delete(self)
    }

    fn auto_register_guilds(&self) -> Vec<GuildId> {
        T::auto_register_guilds(self)
    }
}

impl<T: SettingsProvider> SettingsProvider for RwLock<T> {
    fn default_prefixes(&self) -> Vec<String> {
        async { self.read().await.default_prefixes() }
            .now_or_never()
            .unwrap()
    }

    fn prefixes(&self, guild_id: GuildId) -> Option<Vec<String>> {
        async { self.read().await.prefixes(guild_id) }
            .now_or_never()
            .unwrap()
    }

    fn auto_register(&self) -> bool {
        async { self.read().await.auto_register() }
            .now_or_never()
            .unwrap()
    }

    fn auto_delete(&self) -> bool {
        async { self.read().await.auto_delete() }
            .now_or_never()
            .unwrap()
    }

    fn auto_register_guilds(&self) -> Vec<GuildId> {
        async { self.read().await.auto_register_guilds() }
            .now_or_never()
            .unwrap()
    }
}

impl<T: SettingsProvider> SettingsProvider for StdRwLock<T> {
    fn default_prefixes(&self) -> Vec<String> {
        T::default_prefixes(&self.read().unwrap())
    }

    fn prefixes(&self, guild_id: GuildId) -> Option<Vec<String>> {
        T::prefixes(&self.read().unwrap(), guild_id)
    }

    fn auto_register(&self) -> bool {
        T::auto_register(&self.read().unwrap())
    }

    fn auto_delete(&self) -> bool {
        T::auto_delete(&self.read().unwrap())
    }

    fn auto_register_guilds(&self) -> Vec<GuildId> {
        T::auto_register_guilds(&self.read().unwrap())
    }
}
