enum MyError  {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

use std::fmt;
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O {}",cause),
            MyError::Num(cause) => write!(f, "Parse Error {}",cause),
        }
    }
}

fn get_int_from_file()-> Result<i32, MyError> {
    let path = "number.txt";

    // ? はResult型を返す関数で使える演算子であり、
    // 「その直前の結果のResult型の値がOk(t)であればtを返し、Err(e)であればErr(e)で早期リターンして関数を終了する」動作
    let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;
    
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e))

}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => match e {
            MyError::Io(cause) => println!("I/O {}",cause),
            MyError::Num(cause) => println!("Parse Error {}",cause),
        },
    }
}