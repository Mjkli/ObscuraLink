
pub mod dictionary;
use dictionary::Dictionary;
use std::path::Path;

fn main() {

    let names = Path::new("bin/first-names.txt");
    let mut dic = Dictionary::new(names);
    dic.print();
    let out = dic.random();
    println!("out: {:?}", out);

}
