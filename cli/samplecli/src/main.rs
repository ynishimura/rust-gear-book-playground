use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Parser,Debug)]
#[clap(
    name = "My RPN probram",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    // set the level of verbosity
    #[clap(short,long)]
    verbose: bool,

    // // Number
    // #[clap(name = "NUMBER")]
    // num: i32,

    // Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}
fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
            }
    // match opts.formula_file {
    //     Some(file) => println!("File specified: {}", file),
    //     None => println!("No file specified."),
    // }
    // println!("Is verbosity specified?: {}", opts.verbose);
}

fn run<Reader: BufRead>(reader: Reader, verbose:bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}