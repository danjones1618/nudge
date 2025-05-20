use clap::CommandFactory;
// use colored::Colorize;
use http::Method;
use nudge::{
    self,
    cli::{CliArgs, Commands},
};

fn main() {
    let args = nudge::cli::parse_cli_args();

    match args.command {
        Commands::Completions { shell } => {
            shell.generate(&mut CliArgs::command(), &mut std::io::stdout());
            return;
        }
        _ => (),
    }

    println!("{:#?}", args);
    let spec = nudge::load_spec();

    println!("{:#?}", spec.operation(&Method::POST, "/pet/{petId}"));

    let op_ids: Vec<_> = spec
        .operations()
        .map(|op| (op.0, op.1, op.2.tags.clone()))
        .collect();
    println!("{:#?}", op_ids);
    println!("{:#?}", spec.servers);
}
