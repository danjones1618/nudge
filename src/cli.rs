use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate the API client
    Generate {
        /// Generate a a HTTP client in the target language
        #[arg(long, value_enum, default_value_t = TargetLanguage::Python)]
        language: TargetLanguage,
    },
    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TargetLanguage {
    Python,
}

pub fn parse_cli_args() -> CliArgs {
    CliArgs::parse()
}
