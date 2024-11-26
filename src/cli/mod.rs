use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("Anya Core")
        .version("0.1.0")
        .author("Anya Core Contributors")
        .about("A decentralized AI assistant framework")
        .subcommand(SubCommand::with_name("start")
            .about("Starts the Anya Core daemon"))
        .subcommand(SubCommand::with_name("stop")
            .about("Stops the Anya Core daemon"))
        // Add more subcommands as needed
}