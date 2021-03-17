use std::collections::HashMap;

pub struct Obj {
    sum: u64,
    name: String,
    map: HashMap<u64, u64>,
}

impl Obj {
    fn new() -> Self {
        Obj {
            sum: 10,
            name: "name".to_string(),
            map: Default::default(),
        }
    }

    pub fn check(&self) {
        match self {
            Obj {
                sum: a,
                name: b,
                map: c,
            } => {
                {
                    println!("{}", a);
                }
                {
                    println!("{}", b);
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    Obj::new().check()
}
