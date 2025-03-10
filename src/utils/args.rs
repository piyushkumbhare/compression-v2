use clap::{Arg, ArgGroup, Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum Mode {
    /// Compress a file to .pkz format
    Compress {
        file: String,

        /// Specify a custom pipeline in a comma separated list (Default: -p=BWT,MTF,RLE,HUFF)
        #[arg(short, long, value_delimiter = ',', num_args = 1..)]
        pipeline: Option<Vec<String>>,

        /// After compressing, perform a dry-run decompress to verify that the file will decompress
        /// correctly
        #[arg(short, long, default_value_t = false)]
        check_integerity: bool,

        /// Specify an output file (Default: <FILE>.pkz)
        #[arg(short, long = "output-file")]
        output: Option<String>,
    },
    /// Decompress a .pkz file
    Decompress {
        file: String,

        /// Specify an output file (Default: Strips .pkz suffix if possible, else <FILE>.dcmp)
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

    /// Be verbose
    #[clap(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Be quiet
    #[clap(short, long, default_value_t = false)]
    pub quiet: bool,
}
