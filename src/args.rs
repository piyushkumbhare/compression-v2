use clap::{Arg, ArgGroup, Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum Mode {
    Compress {
        file: String,

        #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
        pipeline: Option<Vec<String>>,

        #[arg(short, long, default_value_t = false)]
        check_integerity: bool,

        #[arg(short, long = "output-file")]
        output: Option<String>,
    },
    Decompress {
        file: String,

        #[arg(short, long = "output-file")]
        output: Option<String>,
    },
}

#[derive(Parser, Debug, Clone)]
#[clap(group(
    ArgGroup::new("log").args(&["verbose", "quiet"])
))]
pub struct Args {
    #[command(subcommand)]
    pub command: Mode,

    #[clap(short, long, default_value_t = false)]
    pub verbose: bool,

    #[clap(short, long, default_value_t = false)]
    pub quiet: bool,
}
