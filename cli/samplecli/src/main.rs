use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

//　今回はオプションが一つしかないので、無名で作成
struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose:bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax");
        }
    }
}

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
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}