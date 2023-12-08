
// <------ SQL Actions ------>
pub struct Select;
pub struct Insert;



#[derive(Debug)]
pub enum Actions {
    SelectAll,
    InsertInto,
}

#[derive(Debug)]
pub struct Response {
    pub request_type: Actions,
    pub response: Vec<String>
}

pub struct Cacher {
    pub data: Vec<Response>,
}


impl Cacher {
    pub fn init() -> Cacher {
        let mut data: Vec<Response> = Vec::new();
        Cacher { data }
    }

    pub fn insert(&mut self, r: Response)  { self.data.push(r); }
    pub fn print(&self) { println!("{:?}", self.data); }
}
