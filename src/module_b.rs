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
}
