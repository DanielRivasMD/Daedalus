////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod completion {

    use clap::{Command, CommandFactory};
    use clap_complete::{generate, shells::*};
    use std::io;

    use crate::cli;

    pub fn run(shell: cli::Shell) -> super::anyResult<()> {
        let visible: Vec<_> = cli::Cli::command()
            .get_subcommands()
            .filter(|s| !s.is_hide_set())
            .cloned()
            .collect();

        let mut cmd = Command::new(env!("CARGO_BIN_NAME")).subcommands(visible);

        let name = cmd.get_name().to_string();

        match shell {
            cli::Shell::Bash => generate(Bash, &mut cmd, name, &mut io::stdout()),
            cli::Shell::Zsh => generate(Zsh, &mut cmd, name, &mut io::stdout()),
            cli::Shell::Fish => generate(Fish, &mut cmd, name, &mut io::stdout()),
            cli::Shell::Powershell => generate(PowerShell, &mut cmd, name, &mut io::stdout()),
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod identity {
    const IDENTITY: &str = r#"
In Greek mythology, Daedalus (UK: /ˈdiːdələs/, US: /ˈdɛdələs/; Greek:
Δαίδαλος; Latin: Daedalus; Etruscan: Taitale) was a skillful architect and
craftsman, seen as a symbol of wisdom, knowledge and power. He is the father
of Icarus, the uncle of Perdix, and possibly also the father of Iapyx.
"#;

    pub fn run() -> super::anyResult<()> {
        println!("{}", IDENTITY);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
