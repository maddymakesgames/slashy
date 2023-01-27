#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::{
        commands::CommandResult,
        framework::CommandContext,
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

        let x = test(&CommandContext::new_test(HashMap::new()))
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

        let x = success(&CommandContext::new_test(HashMap::new()))
            .now_or_never()
            .unwrap();
        let y = failure(&CommandContext::new_test(HashMap::new()))
            .now_or_never()
            .unwrap();

        println!("{x:?}");
        println!("{y:?}");

        if let Ok(b) = x {
            assert!(b)
        } else {
            panic!()
        }


        if let Ok(b) = y {
            assert!(!b)
        } else {
            panic!()
        }
    }
}
