pub fn add(x:i32, y:i32) -> i32 {
    x + y
}

#[test]
fn test_add() {
    assert_eq!(0, add(0,0));
    assert_eq!(1, add(0,1));
    assert_eq!(1, add(1,0));
    assert_eq!(2, add(1,1));
    assert_eq!(0, add(0,0));
}

#[test]
fn assert_sample() {
    // assert!(true);
    // assert!(false, "panic! value={}", false);
    // assert_eq!(true, true);
    // assert_ne!(true, true);
    // assert_eq!(true, false, "panic! value=({}, {})", true, false);
}

#[test]
#[should_panic]
fn test_panic() {
    panic!("expected panic");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add(2,2),4);
    }
}