use serenity::model::{id::UserId, interactions::Interaction};
use slashy::{
    argument::Argument,
    commands::{ArgumentChoice, CommandArguments, CommandArgumentsTree, CommandResult},
    framework::{CommandContext, CommandSource},
    subcommand,
};

#[test]
fn interaction_parse_test() {
    #[subcommand]
    fn test(_ctx: &CommandContext) -> CommandResult {
        println!("test");
        Ok(())
    }

    #[subcommand]
    fn test2(_ctx: &CommandContext) -> CommandResult {
        println!("test2");
        Ok(())
    }

    #[subcommand]
    fn test3(_ctx: &CommandContext) -> CommandResult {
        println!("test3");
        Ok(())
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

    let source1 = serde_json::from_str::<Interaction>(r#"{"id":"0","application_id":0,"type":2,"data":{"options":[{"name":"get","type":1,"options":[{"name":"points","type":1,"options":[{"name":"user","type":4,"value":0}]}]}],"name":"test","id":"0"},"guild_id":"0","channel_id":"0","member":{"user":{"id":0,"username":"","avatar":"a_d5efa99b3eeaa7dd43acca82f5692432","discriminator":"0000","public_flags":0},"roles":["539082325061836999"],"premium_since":null,"permissions":"0","pending":false,"nick":null,"mute":false,"joined_at":"2017-03-13T19:19:14.040000+00:00","is_pending":false,"deaf":false},"token":"otywftnowf","version":1}"#).unwrap();
    let source2 = serde_json::from_str::<Interaction>(r#"{"version":2,"type":2,"application_id":0,"token":"otywftnowf","member":{"user":{"id":0,"username":"","avatar":"a_d5efa99b3eeaa7dd43acca82f5692432","discriminator":"0000","public_flags":0},"roles":["539082325061836999"],"premium_since":null,"permissions":"0","pending":false,"nick":null,"mute":false,"joined_at":"2017-03-13T19:19:14.040000+00:00","is_pending":false,"deaf":false},"id":"0","guild_id":"0","data":{"options":[{"name":"get","type":1,"options":[{"name":"leaderboard","type":1,"options":[{"name":"page","value":0,"type":4}]}]}],"name":"test","id":"0"},"channel_id":"0"}"#).unwrap();
    let source3 = serde_json::from_str::<Interaction>(r#"{"version":2,"type":2,"application_id":0,"token":"otywftnowf","member":{"user":{"id":0,"username":"","avatar":"a_d5efa99b3eeaa7dd43acca82f5692432","discriminator":"0000","public_flags":0},"roles":["539082325061836999"],"premium_since":null,"permissions":"0","pending":false,"nick":null,"mute":false,"joined_at":"2017-03-13T19:19:14.040000+00:00","is_pending":false,"deaf":false},"id":"0","guild_id":"0","data":{"options":[{"name":"self","type":1}],"name":"test","id":"0"},"channel_id":"0"}"#).unwrap();
    let source4 = serde_json::from_str::<Interaction>(r#"{"version":2,"type":2,"application_id":0,"token":"otywftnowf","member":{"user":{"id":0,"username":"","avatar":"a_d5efa99b3eeaa7dd43acca82f5692432","discriminator":"0000","public_flags":0},"roles":["539082325061836999"],"premium_since":null,"permissions":"0","pending":false,"nick":null,"mute":false,"joined_at":"2017-03-13T19:19:14.040000+00:00","is_pending":false,"deaf":false},"id":"0","guild_id":"0","data":{"options":[],"name":"test","id":"0"},"channel_id":"0"}"#).unwrap();

    let args1 = Argument::parse(&CommandSource::Interaction(source1), &arguments_tree);
    let args2 = Argument::parse(&CommandSource::Interaction(source2), &arguments_tree);
    let args3 = Argument::parse(&CommandSource::Interaction(source3), &arguments_tree);
    let args4 = Argument::parse(&CommandSource::Interaction(source4), &arguments_tree);

    assert!(args1.is_some());
    let args = args1.unwrap();
    assert_eq!(args.1 as usize, test as usize);
    assert_eq!(args.0.get("user"), Some(&Argument::User(UserId(0))));

    assert!(args2.is_some());
    let args = args2.unwrap();
    assert_eq!(args.1 as usize, test2 as usize);
    assert_eq!(args.0.get("page"), Some(&Argument::Integer(0)));

    assert!(args3.is_some());
    let args = args3.unwrap();
    assert_eq!(args.1 as usize, test3 as usize);
    assert!(args.0.is_empty());

    assert!(args4.is_none());
}

#[test]
fn message_parse_test() {}
