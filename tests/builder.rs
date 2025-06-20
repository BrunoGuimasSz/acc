use command::command::Command;
use command::flag::Flag;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_command_build() {
        let args = Command::new("linux")
            .description("Linux kernel")
            .subcommand(
                Command::new("ls")
                .description("List files and dirs")
                .flag(
                    Flag::new("a")
                    .description("Show all files")
                    .short('a')
                )
                .flag(
                    Flag::new("l")
                    .description("Show long files")
                    .short('l')
                )
            );

        assert_eq!(args.get_name(), "linux");
        assert_eq!(args.get_description(), Some("Linux kernel"));
        assert_eq!(
            args.get_subcommands()[0].get_name(),
            "ls"
            );
        assert_eq!(
            args.get_subcommands()[0].get_description(),
            Some("List files and dirs")
            );
        assert_eq!(
            args.get_subcommands()[0].get_flags()[0].get_name(),
            "a"
        );
        assert_eq!(
            args.get_subcommands()[0].get_flags()[0].get_description(),
            Some("Show all files")
        );
    }
}

