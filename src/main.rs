use std::string;

fn main() {
    //     println!("Hello, world!");
    //     let objective: Option<i32> = Some(1);
    //     match objective {
    //         Some(x) if x % 2 == 0 => println!("偶数です: {}", x),
    //         Some(x) => println!("奇数です: {}", x),
    //         None => println!("値がありません"),
    //     }
    let dog = Dog {};
    let cat = Cat {};
    show_animal_data(dog);
    show_animal_data(cat);

    let i: i32 = 4;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        _ => println!("misc"),
    }

    let p = Person {
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name().say_age();

    let mut p = HappyPerson {
        name: "John".to_string(),
        state: Emotion::Happy,
    };
    println!("{}", p.get_happy())
}

enum Emotion {
    Anger,
    Happy,
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tall_state(&mut self) -> String;
}

struct HappyPerson {
    name: String,
    state: Emotion,
}

impl Emotional for HappyPerson {
    fn get_anger(&mut self) -> String {
        unimplemented!()
    }
    fn get_happy(&mut self) -> String {
        format!("{} is always happy.", self.name)
    }
    fn tall_state(&mut self) -> String {
        todo!()
    }
}
struct Person {
    name: String,
    age: u32,
}
impl Person {
    fn say_name(&self) -> &Self {
        println!("I am {}", self.name);
        self
    }
    fn say_age(&self) -> &Self {
        println!("I am {} yesars old.", self.age);
        self
    }
}

struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}
trait Animal {
    // トレイトは型クラス
    fn lifespan(&self) -> u32;
    fn scientific_name(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }

    fn scientific_name(&self) -> String {
        "Canis lupus familiaris".to_string()
    }
}

struct Cat;
impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        16
    }
    fn scientific_name(&self) -> String {
        "Felis".to_string()
    }
}

fn show_animal_data<T: Animal>(animal: T) {
    println!("Lifespan: {} years", animal.lifespan());
    println!("Scientific name: {}", animal.scientific_name());
}
