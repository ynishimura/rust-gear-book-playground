trait Tweet {
    fn tweet(&self) {}
    fn twice_tweet(&self) {
        self.tweet();
        self.tweet();
    }
    fn shout(&self) {
        println!("fooooooooo");
    }
}

struct Duck;
struct Dove;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

pub fn func_b() {
    let dove = Dove {};
    dove.tweet();
    dove.twice_tweet();
    dove.shout();

    let duck = Duck {};

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }

    let mut important_data = "Hello World".to_string();

    calc_data(&important_data);
    println!("{}", &important_data);

    let mut x = 5;
    let y = &x;
    let z = &mut x;

    dbg!(z);
    dbg!(x);
}

struct Color {
    r: i32,
    g: i32,
    b: i32,
}

fn calc_data(data: &String) {
    println!("{}", data);
}
