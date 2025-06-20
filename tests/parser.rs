use command::command::Command;
use command::flag::Flag;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        let args = vec![
            "acc".to_string(),
            "linux".to_string(),
            "ls".to_string(),
            "-a".to_string(),
            "-l".to_string(),
            "root/".to_string(),
        ];
        let parsed = Command::new("linux")
            .description("Linux kernel")
            .subcommand(
                Command::new("ls")
                    .description("List files and dirs")
                    .flag(Flag::new("a").description("Show all files").short('a'))
                    .flag(Flag::new("l").description("Show long files").short('l')),
            )
            .parse(args);

        assert_eq!(parsed.command, "linux".to_string());
        assert_eq!(parsed.subcommand, Some("ls".to_string()));
        assert_eq!(parsed.flag[0], "-a".to_string());
        assert_eq!(parsed.flag[1], "-l".to_string());
        assert_eq!(parsed.value, Some("root/".to_string()));
        assert_eq!(
            parsed.arg_vec,
            vec![
                "acc".to_string(),
                "linux".to_string(),
                "ls".to_string(),
                "-a".to_string(),
                "-l".to_string(),
                "root/".to_string(),
                ]
            )
    }

    #[test]
    fn no_subcommand() {
        let args = vec![
            "acc".to_string(),
            "linux".to_string(),
            "-a".to_string(),
            "-l".to_string(),
            "home/".to_string(),
            ];
            let parsed = Command::new("linux")
            .description("Linux kernel")
            .subcommand(
                Command::new("ls")
                .description("List files and dirs")
                    .flag(Flag::new("a").description("Show all files").short('a'))
                    .flag(Flag::new("l").description("Show long files").short('l')),
                )
                .parse(args);

        assert_eq!(parsed.command, "linux".to_string());
        assert_eq!(parsed.subcommand, None);
        assert_eq!(parsed.flag[0], "-a".to_string());
        assert_eq!(parsed.flag[1], "-l".to_string());
        assert_eq!(parsed.value, Some("home/".to_string()));
    }

    #[test]
    fn no_value() {
      let args = vec![
            "acc".to_string(),
            "linux".to_string(),
            "-a".to_string(),
            "-l".to_string(),
            ];
            let parsed = Command::new("linux")
            .description("Linux kernel")
            .subcommand(
                Command::new("ls")
                .description("List files and dirs")
                    .flag(Flag::new("a").description("Show all files").short('a'))
                    .flag(Flag::new("l").description("Show long files").short('l')),
                )
                .parse(args);

        assert_eq!(parsed.command, "linux".to_string());
        assert_eq!(parsed.subcommand, None);
        assert_eq!(parsed.flag[0], "-a".to_string());
        assert_eq!(parsed.flag[1], "-l".to_string());
        assert_eq!(parsed.value, None);
    }

    #[test]
    #[should_panic]
    fn no_flags() {
              let args = vec![
            "acc".to_string(),
            "linux".to_string(),
            "home/".to_string(),
            ];
            let parsed = Command::new("linux")
            .description("Linux kernel")
            .subcommand(
                Command::new("ls")
                .description("List files and dirs")
                    .flag(Flag::new("a").description("Show all files").short('a'))
                    .flag(Flag::new("l").description("Show long files").short('l')),
                )
                .parse(args);

        assert_eq!(parsed.command, "linux".to_string());
        assert_eq!(parsed.subcommand, None);
        assert_eq!(parsed.flag[0], "-a".to_string());
        assert_eq!(parsed.value, Some("home/".to_string()));
    }

    #[test]
    #[should_panic]
    fn wrong_command() {
              let args = vec![
            "acc".to_string(),
            "mac".to_string(),
            "-a".to_string(),
            "-l".to_string(),
            "home/".to_string(),
            ];
            let parsed = Command::new("linux")
            .description("Linux kernel")
            .subcommand(
                Command::new("ls")
                .description("List files and dirs")
                    .flag(Flag::new("a").description("Show all files").short('a'))
                    .flag(Flag::new("l").description("Show long files").short('l')),
                )
                .parse(args);

        assert_eq!(parsed.command, "linux".to_string());
        assert_eq!(parsed.subcommand, None);
        assert_eq!(parsed.flag[0], "-a".to_string());
        assert_eq!(parsed.flag[1], "-l".to_string());
        assert_eq!(parsed.value, Some("home/".to_string()));
    }
}
