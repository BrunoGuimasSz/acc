use command::{app::App, command::Command, flag::Flag};

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME Corrigir esse teste arrumando o get_flag() 
    #[test]
    fn full() {
        let args = vec![
            "acc".to_string(),
            "mac".to_string(),
            "ls".to_string(),
            "-a".to_string(),
            "-l".to_string(),
            "home/".to_string(),
        ];

        let cli = App::new()
            .command(
                Command::new("linux")
                    .description("Linux kernel")
                    .subcommand(
                        Command::new("ls")
                            .description("List files and dirs")
                            .flag(Flag::new("all").description("Show all files").short('a'))
                            .flag(Flag::new("list").description("Show long files").short('l')),
                    ),
            )
            .command(
                Command::new("mac").description("Mac kernel").subcommand(
                    Command::new("ls")
                        .description("List files and dirs")
                        .flag(Flag::new("all").description("Show all files").short('a'))
                        .flag(Flag::new("list").description("Show long files").short('l')),
                ),
            );

        let parsed = cli.parse(args).unwrap();

        assert_eq!(parsed.command, "mac".to_string());
        assert_eq!(parsed.subcommand, Some("ls".to_string()));
        assert_eq!(parsed.flag[0], "-a".to_string());
        assert_eq!(parsed.flag[1], "-l".to_string());
        assert_eq!(parsed.value, Some("home/".to_string()));
    }

    #[test]
    #[should_panic(expected = "Error: Command not found: windows")]
    fn wrong_command() {
        let args = vec![
            "acc".to_string(),
            "windows".to_string(),
            "ls".to_string(),
            "-a".to_string(),
            "-l".to_string(),
            "home/".to_string(),
        ];

        let cli = App::new()
            .command(
                Command::new("linux")
                    .description("Linux kernel")
                    .subcommand(
                        Command::new("ls")
                            .description("List files and dirs")
                            .flag(Flag::new("all").description("Show all files").short('a'))
                            .flag(Flag::new("list").description("Show long files").short('l')),
                    ),
            )
            .command(
                Command::new("mac").description("Mac kernel").subcommand(
                    Command::new("ls")
                        .description("List files and dirs")
                        .flag(Flag::new("all").description("Show all files").short('a'))
                        .flag(Flag::new("list").description("Show long files").short('l')),
                ),
            );

        let parsed = cli.parse(args).unwrap();

        assert_eq!(parsed.command, "mac".to_string());
        assert_eq!(parsed.subcommand, Some("ls".to_string()));
        assert_eq!(parsed.flag[0], "-a".to_string());
        assert_eq!(parsed.flag[1], "-l".to_string());
        assert_eq!(parsed.value, Some("home/".to_string()));
    }
}
