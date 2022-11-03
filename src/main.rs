
fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("expected key");
    let value = args.next().expect("expected value");
    println!("the key {} has the value {}", key, value);
    let contents = format!("{}\t{}\n", key, value);
    let write_result = std::fs::write("kv.db", contents);
    
}

