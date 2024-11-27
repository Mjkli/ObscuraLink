
mod dictionary;

use dictionary::Dictionary;


fn main() {
    
    let mut dic = Dictionary::new();

    let words: Vec<String> = ["test", "string", "here"].map(String::from).to_vec();

    dic.update_words(words);
    
    dic.print();

}
