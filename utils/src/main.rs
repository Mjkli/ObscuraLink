
mod dictionary;

use dictionary::Dictionary;


fn main() {    
    let mut dic = Dictionary::new();
    dic.update_words(["test", "string", "here"].map(String::from).to_vec());
    dic.print();
    let out = dic.random();
    println!("out: {:?}", out);

}
