async fn add(left: i32, right: i32) -> i32 {
    left + right
    
}

#[tokio::main]
pub async fn func_g() {
    let ans = add(2, 3).await;
    println!("{}", ans);
}