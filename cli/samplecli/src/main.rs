use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    #[clap(name = "FILE", default_value = "default.txt")]
    formula_file: String,
}
fn main() {
    let opts = Opts::parse();

    if let path = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
                // not file
                println!("No file is specified");
            }
    // match opts.formula_file {
    //     Some(file) => println!("File specified: {}", file),
    //     None => println!("No file specified."),
    // }
    // println!("Is verbosity specified?: {}", opts.verbose);

}
