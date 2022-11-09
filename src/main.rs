use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("expected key");
    let value = args.next().expect("expected value");
    println!("the key {} has the value {}", key, value);
    
    let mut db = Database::new().expect("creating db failed");
    db.insert(key.to_uppercase(), value.clone());
    db.insert(key, value.clone());
    db.flush().unwrap();
    // method is just syntactic sugar for functions
    // same as Database::insert(db, key, value);
}


struct Database {
    map: HashMap<String, String>
}

impl Database {

    // new -> lowkey like contructor
    fn new() -> Result<Database, std::io::Error> {
        //read from kv.db
        let mut map = HashMap::new();
        let contents  = std::fs::read_to_string("kv.db")?;
        println!("before\n{contents}");
        let content = contents.lines();
        for line in content{
            let mut chunks = line.splitn(2, "\t");
            let key = chunks.next().expect("key not found");
            let value = chunks.next().expect("value not found");
            map.insert(key.to_owned(), value.to_owned());
            
        };
        Ok(Database { 
            map
        })
    }


    // insert -> inserts stuff into db
    fn insert(&mut self, key:String, value:String){
        self.map.insert(key, value);
    }


    fn flush(self) -> std::io::Result<()>{
        let mut contents = String::new();
        for (key, value) in self.map {
            contents.push_str(&key);
            contents.push('\t');
            contents.push_str(&value);
            contents.push('\n');
        }
        std::fs::write("kv.db", contents)
    }
    

}
