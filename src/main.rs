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
