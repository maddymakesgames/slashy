#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::{
        commands::CommandResult,
        framework::{CommandContext, CommandSource},
        permissions::PermsResult,
        permissions_check,
        subcommand,
    };
    use serenity::FutureExt;

    #[test]
    fn subcommand() {
        #[subcommand]
        async fn test(_ctx: &CommandContext) -> CommandResult<u32> {
            Ok(5)
        }

        let x = test(&CommandContext::new(
            CommandSource::Test(""),
            HashMap::new(),
        ))
        .now_or_never()
        .unwrap()
        .unwrap();
        assert_eq!(x, 5);
    }

    #[test]
    fn perms_check() {
        #[permissions_check]
        pub fn pass() -> PermsResult {
            Ok(true)
        }

        #[permissions_check]
        pub fn fail() -> PermsResult {
            Ok(false)
        }

        #[subcommand(pass)]
        async fn success(_ctx: &CommandContext) -> CommandResult<bool> {
            Ok(true)
        }

        #[subcommand(fail)]
        async fn failure(_ctx: &CommandContext) -> CommandResult<bool> {
            Ok(true)
        }

        let x = success(&CommandContext::new(CommandSource::Test(""), HashMap::new())).now_or_never().unwrap();
        let y = failure(&CommandContext::new(CommandSource::Test(""), HashMap::new())).now_or_never().unwrap();

        println!("{:?}", x);
        println!("{:?}", y);

        match x {
            Ok(b) => assert!(b),
            Err(_) => assert!(false)
        }


        match y {
            Ok(b) => assert!(!b),
            Err(_) => assert!(true)
        }
    }
}
