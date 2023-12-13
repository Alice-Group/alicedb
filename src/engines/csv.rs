pub trait Engine {
    fn init();
}

pub struct CSVEngine;
pub struct PostgresEngine;
pub struct JsonEngine;

impl Engine for CSVEngine {
    fn init() {
        todo!();
    }
}

impl Engine for PostgresEngine {
    fn init() {
        todo!();
    }
}

impl Engine for JsonEngine {
    fn init() {
        todo!();
    }
}