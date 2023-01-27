use slashy::{
    argument::Argument,
    command,
    commands::CommandResult,
    framework::CommandContext,
    subcommand,
};

#[test]
fn command_macro_test() {
    #[subcommand]
    fn test(_cmd: &CommandContext) -> CommandResult {
        Ok(())
    }

    command! {
        test,
        "test command",
        test,
        [
            required Integer test | "test int"
        ]
    }

    let cmd: Command = TEST_COMMAND::command_init();
    assert_eq!("test", cmd.name);
    assert_eq!("test command", cmd.description);

    let args = Argument::parse_message("test 12", &cmd.arguments_tree).unwrap();
    assert_eq!(Some(&Argument::Integer(12)), args.0.get("test"));
    assert_eq!(None, args.0.get("testt"));
}
