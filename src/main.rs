pub mod get_coverage;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file or path with coverage files. Can be zipped.
    #[arg(short, long)]
    input: String,

    /// Reference file to get the length of the chromosomes to check his name.
    #[arg(short, long)]
    reference: String,

    /// Output file name.
    #[arg(short, long)]
    output: String,
    ///
    /// Define the ratios to output, separated by comma. Ex: '0,4,9'. Default: '0,9'. The cutoff ratios is more than the values defined."
    #[arg(long, default_value_t = String::from("0,9") )]
    ratio: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    
    get_coverage::coverage::process_files(args.input, args.reference, args.output, args.ratio)
}
