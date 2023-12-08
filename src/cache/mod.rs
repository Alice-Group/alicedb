use std::collections::HashMap;

pub struct Cache {
    pub data: HashMap<String, String>
}

impl Cache {
    pub fn init() -> Cache {
        let hm: HashMap<String, String> = HashMap::new();
        Cache { data: hm }
    }

    pub fn insert(&mut self) {
        todo!();
        //self.data.insert()
    }
}