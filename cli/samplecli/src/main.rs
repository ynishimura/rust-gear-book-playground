use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

enum MyError  {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}
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
            if let Ok(x) = token.parse::<i32>() { // 取り出した値が数値のときpush
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

            if self.0 { // -vを指定したかどうか判定するため、RpnCalculatorの持つ無名のフィールドを取得、self.0は何番目のフィールドかを表す
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("50"), 50);
        assert_eq!(calc.eval("-50"), -50);
        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);
        assert_eq!(2 * 2, 4);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalculator::new(false);

        calc.eval("1 1 ^");        
    }
}