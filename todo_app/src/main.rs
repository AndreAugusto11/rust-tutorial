use std::env;
use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}

fn main() {
    let action = env::args().nth(1).expect("Please specify an action");
    let item = env::args().nth(2).expect("Please specify an item");
    
    println!("Inserting {:?}, {:?}", action, item);

    let mut todo = Todo {
        map: HashMap::new()
    };
}
